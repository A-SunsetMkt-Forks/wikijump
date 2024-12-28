<script lang="ts">
  import { page } from "$app/stores"
  import { goto, invalidateAll } from "$app/navigation"
  import { onMount } from "svelte"
  import { useErrorPopup, usePagePaneState } from "$lib/stores"
  import { PagePane } from "$lib/types"
  import { EditorPane, FilePane, LayoutPane, MovePane, ParentPane, VotePane } from "."
  let showErrorPopup = useErrorPopup()
  let pagePaneState = usePagePaneState()

  let showHistory = false
  let showSource = false
  let showRevision = false
  let showRevisionSource = false
  let revisionMap: Map<number, Record<string, any>> = new Map()
  let revision: Record<string, any> = {}

  async function handleDelete() {
    let fdata = new FormData()
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    fdata.set("last-revision-id", $page.data.page_revision.revision_id)
    let res = await fetch(`/${$page.data.page.slug}`, {
      method: "DELETE",
      body: fdata
    }).then((res) => res.json())
    if (res?.message) {
      showErrorPopup.set({
        state: true,
        message: res.message,
        data: res.data
      })
    } else invalidateAll()
  }

  function navigateEdit() {
    let options: string[] = []
    if ($page.data.options.no_render) options.push("norender")
    options = options.map((opt) => `/${opt}`)
    goto(`/${$page.data.page.slug}${options.join("")}/edit`, {
      noScroll: true
    })
  }

  async function handleHistory() {
    let fdata = new FormData()
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    let res = await fetch(`/${$page.data.page.slug}/history`, {
      method: "POST",
      body: fdata
    }).then((res) => res.json())
    if (res?.message) {
      showErrorPopup.set({
        state: true,
        message: res.message,
        data: res.data
      })
    } else {
      res.forEach((rev) => {
        revisionMap.set(rev.revision_number, rev)
      })
      showHistory = true
    }
  }

  async function getRevision(
    revisionNumber: number,
    compiledHtml: boolean,
    wikitext: boolean
  ) {
    // Get cached revision if we have it
    let rev = revisionMap.get(revisionNumber)
    // Try to see if the cached revision already has the wanted data
    if (compiledHtml && rev?.compiled_html) {
      revision = rev
    } else if (wikitext && rev?.wikitext) {
      revision = rev
    } else {
      // Request from server
      let fdata = new FormData()
      fdata.set("site-id", $page.data.site.site_id)
      fdata.set("page-id", $page.data.page.page_id)
      fdata.set("revision-number", revisionNumber)
      fdata.set("compiled-html", compiledHtml)
      fdata.set("wikitext", wikitext)
      let res = await fetch(`/${$page.data.page.slug}/revision`, {
        method: "POST",
        body: fdata
      }).then((res) => res.json())
      if (res?.message) {
        showErrorPopup.set({
          state: true,
          message: res.message,
          data: res.data
        })
      } else if (!rev) {
        // This is a revision we didn't even cache...?
        revisionMap.set(res.revision_number, res)
        revision = res
      } else if (compiledHtml) {
        rev.compiled_html = res.compiled_html
        revision = rev
      } else if (wikitext) {
        rev.wikitext = res.wikitext
        revision = rev
      }
    }
  }

  async function rollbackRevision(revisionNumber: number, comments?: string) {
    let fdata = new FormData()
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    fdata.set("revision-number", revisionNumber)
    fdata.set("last-revision-id", $page.data.page_revision.revision_id)
    if (comments !== undefined) fdata.set("comments", comments)
    let res = await fetch(`/${$page.data.page.slug}/rollback`, {
      method: "POST",
      body: fdata
    }).then((res) => res.json())
    if (res?.message) {
      showErrorPopup.set({
        state: true,
        message: res.message,
        data: res.data
      })
    } else invalidateAll()
  }

  onMount(() => {
    if ($page.data?.options.history) handleHistory()
  })
</script>

<h1>UNTRANSLATED:Loaded page</h1>
<p>
  UNTRANSLATED:Response <textarea class="debug">{JSON.stringify($page, null, 2)}</textarea
  >
</p>

{#if showRevision}
  <h2>{revision.title}</h2>
{:else}
  <h2>{$page.data.page_revision.title}</h2>
{/if}

<hr />

<div class="page-content">
  {#if $page.data.options?.no_render}
    {$page.data.internationalization["wiki-page-no-render"]}
    <textarea class="page-source" readonly={true}>{$page.data.wikitext}</textarea>
  {:else if showRevision}
    {@html revision.compiled_html}
  {:else}
    {@html $page.data.compiled_html}
  {/if}
</div>

<div class="page-tags-container">
  {$page.data.internationalization?.tags}
  <hr />
  <ul class="page-tags">
    {#if showRevision}
      {#each revision.tags as tag}
        <li class="tag">{tag}</li>
      {/each}
    {:else}
      {#each $page.data.page_revision.tags as tag}
        <li class="tag">{tag}</li>
      {/each}
    {/if}
  </ul>
</div>

<div class="page-meta-info-container">
  <div class="page-meta-info info-revision">
    {$page.data.internationalization["wiki-page-revision"]}
  </div>
  <div class="page-meta-info info-last-edit">
    {$page.data.internationalization["wiki-page-last-edit"]}
  </div>
</div>

{#if $page.data.options?.edit}
  <EditorPane />
{:else}
  <div class="action-row editor-actions">
    <button
      class="action-button editor-button button-move clickable"
      type="button"
      on:click={() => {
        pagePaneState.set(PagePane.Move)
      }}
    >
      {$page.data.internationalization?.move}
    </button>
    <button
      class="action-button editor-button button-layout clickable"
      type="button"
      on:click={() => {
        pagePaneState.set(PagePane.Layout)
      }}
    >
      {$page.data.internationalization?.layout}
    </button>
    <button
      class="action-button editor-button button-parents clickable"
      type="button"
      on:click={() => {
        pagePaneState.set(PagePane.Parent)
      }}
    >
      {$page.data.internationalization?.parents}
    </button>
    <button
      class="action-button editor-button button-delete clickable"
      type="button"
      on:click={handleDelete}
    >
      {$page.data.internationalization?.delete}
    </button>
    <button
      class="action-button editor-button button-edit clickable"
      type="button"
      on:click={navigateEdit}
    >
      {$page.data.internationalization?.edit}
    </button>
  </div>
  <div class="action-row other-actions">
    <button
      class="action-button button-source clickable"
      type="button"
      on:click={() => (showSource = true)}
    >
      {$page.data.internationalization?.["wiki-page-view-source"]}
    </button>
    <button
      class="action-button button-history clickable"
      type="button"
      on:click={handleHistory}
    >
      {$page.data.internationalization?.history}
    </button>
    <button
      class="action-button button-vote clickable"
      type="button"
      on:click={() => {
        pagePaneState.set(PagePane.Vote)
      }}
    >
      {$page.data.internationalization?.vote}
    </button>
    <button
      class="action-button button-files clickable"
      type="button"
      on:click={() => {
        pagePaneState.set(PagePane.File)
      }}
    >
      {$page.data.internationalization?.files}
    </button>
  </div>
{/if}

{#if showSource}
  <textarea class="page-source" readonly={true}>{$page.data.wikitext}</textarea>
{/if}

{#if $pagePaneState === PagePane.Move}
  <MovePane />
{:else if $pagePaneState === PagePane.Layout}
  <LayoutPane />
{:else if $pagePaneState === PagePane.Parent}
  <ParentPane />
{:else if $pagePaneState === PagePane.Vote}
  <VotePane />
{:else if $pagePaneState === PagePane.File}
  <FilePane />
{/if}

{#if showHistory}
  <div class="revision-list">
    <div class="revision-header">
      <div class="revision-attribute action" />
      <div class="revision-attribute revision-number">
        {$page.data.internationalization?.["wiki-page-revision-number"]}
      </div>
      <div class="revision-attribute revision-type">
        {$page.data.internationalization?.["wiki-page-revision-type"]}
      </div>
      <div class="revision-attribute created-at">
        {$page.data.internationalization?.["wiki-page-revision-created-at"]}
      </div>
      <div class="revision-attribute user">
        {$page.data.internationalization?.["wiki-page-revision-user"]}
      </div>
      <div class="revision-attribute comments">
        {$page.data.internationalization?.["wiki-page-revision-comments"]}
      </div>
    </div>
    <!-- Here we sort the list in descending order. -->
    {#each [...revisionMap].sort((a, b) => b[0] - a[0]) as [_, revisionItem] (revisionItem.revision_number)}
      <div class="revision-row" data-id={revisionItem.revision_id}>
        <div class="revision-attribute action">
          {#if ["create", "regular"].includes(revisionItem.revision_type)}
            <button
              class="action-button view-revision clickable"
              type="button"
              on:click|stopPropagation={() => {
                getRevision(revisionItem.revision_number, true, false).then(() => {
                  showRevision = true
                  showRevisionSource = false
                })
              }}
            >
              {$page.data.internationalization?.view}
            </button>
            <button
              class="action-button view-revision-source clickable"
              type="button"
              on:click|stopPropagation={() => {
                getRevision(revisionItem.revision_number, false, true).then(() => {
                  showRevision = false
                  showRevisionSource = true
                })
              }}
            >
              {$page.data.internationalization?.["wiki-page-view-source"]}
            </button>
            <button
              class="action-button revision-rollback clickable"
              type="button"
              on:click|stopPropagation={() => {
                rollbackRevision(revisionItem.revision_number)
              }}
            >
              {$page.data.internationalization?.["wiki-page-revision-rollback"]}
            </button>
          {/if}
        </div>
        <div class="revision-attribute revision-number">
          {revisionItem.revision_number}
        </div>
        <div class="revision-attribute revision-type">
          {$page.data.internationalization?.[
            `wiki-page-revision-type.${revisionItem.revision_type}`
          ]}
        </div>
        <div class="revision-attribute created-at">
          {new Date(revisionItem.created_at).toLocaleString()}
        </div>
        <div class="revision-attribute user">
          {revisionItem.user_id}
        </div>
        <div class="revision-attribute comments">
          {revisionItem.comments}
        </div>
      </div>
    {/each}
  </div>

  {#if showRevisionSource}
    <textarea class="revision-source" readonly={true}>{revision.wikitext}</textarea>
  {/if}
{/if}

<style global lang="scss">
  .debug {
    width: 80vw;
    height: 60vh;
  }

  .page-content,
  .page-tags-container,
  .page-meta-info-container,
  .editor-actions,
  .other-actions {
    padding: 0 0 2em;
  }

  .page-tags {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    gap: 10px;
    align-items: center;
    justify-content: flex-start;
    padding: 0;
    margin: 0;
    list-style: none;
  }

  .page-meta-info-container {
    text-align: right;
  }

  .page-source,
  .revision-source {
    width: 80vw;
    height: 60vh;
  }

  .action-row {
    display: flex;
    flex-direction: row;
    gap: 10px;
    align-items: stretch;
    justify-content: flex-end;
    width: 100%;
  }

  .revision-list {
    display: table;
    width: 100%;

    .revision-header,
    .revision-row {
      display: table-row;

      .revision-attribute {
        display: table-cell;
      }
    }
  }
</style>
