<script lang="ts">
  import { page } from "$app/stores"
  import { goto, invalidateAll } from "$app/navigation"
  import { onMount } from "svelte"
  import { useErrorPopup } from "$lib/stores"
  import { Layout } from "$lib/types"
  let showErrorPopup = useErrorPopup()

  let showMoveAction = false
  let showLayoutAction = false
  let showParentAction = false
  let showHistory = false
  let showSource = false
  let showRevision = false
  let showRevisionSource = false
  let showVote = false
  let showVoteList = false
  let showFiles = false
  let showFileUploadAction = false
  let showFileEditAction = false
  let showFileMoveAction = false
  let showFileRestoreAction = false
  let moveInputNewSlugElem: HTMLInputElement
  let revisionMap: Map<number, Record<string, any>> = new Map()
  let revision: Record<string, any> = {}
  let voteMap: Map<number, Record<string, any>> = new Map()
  let voteRating: number
  let parents = ""
  let fileMap: Map<number, Record<string, any>> = new Map()
  let filesUpload: FileList
  let filesEditElem: HTMLInputElement
  let fileEditId: number | null = null

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

  function cancelEdit() {
    let options: string[] = []
    if ($page.data.options.no_render) options.push("norender")
    options = options.map((opt) => `/${opt}`)
    goto(`/${$page.data.page.slug}${options.join("")}`, {
      noScroll: true
    })
  }

  async function saveEdit() {
    let form = document.getElementById("editor")
    let fdata = new FormData(form)
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    fdata.set("last-revision-id", $page.data.page_revision.revision_id)
    let res = await fetch(`/${$page.data.page.slug}/edit`, {
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
      goto(`/${$page.data.page.slug}`, {
        noScroll: true
      })
    }
  }

  async function handleMove() {
    let form = document.getElementById("page-move")
    let fdata = new FormData(form)
    let newSlug = fdata.get("new-slug")
    if (!newSlug) {
      moveInputNewSlugElem.classList.add("error")
      return
    } else {
      moveInputNewSlugElem.classList.remove("error")
    }
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    fdata.set("last-revision-id", $page.data.page_revision.revision_id)
    let res = await fetch(`/${$page.data.page.slug}/move`, {
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
      goto(`/${newSlug}`, {
        noScroll: true
      })
      showMoveAction = false
    }
  }

  async function handleLayout() {
    let form = document.getElementById("page-layout")
    let fdata = new FormData(form)
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    let res = await fetch(`/${$page.data.page.slug}/layout`, {
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
      showLayoutAction = false
      invalidateAll()
    }
  }

  async function getParents() {
    let fdata = new FormData()
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    let res = await fetch(`/${$page.data.page.slug}/parent-get`, {
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
      parents = res.join(" ")
      showParentAction = true
    }
  }

  async function setParents() {
    let form = document.getElementById("page-parent")
    let fdata = new FormData(form)
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    let newParents = (fdata.get("parents")?.toString() ?? "").split(" ").filter((p) => p)
    let oldParents = parents.split(" ").filter((p) => p)
    let added: string[] = []
    let removed: string[] = []
    let common: string[] = []
    for (let i = 0; i < oldParents.length; i++) {
      if (!newParents.includes(oldParents[i])) removed.push(oldParents[i])
      else common.push(oldParents[i])
    }
    for (let i = 0; i < newParents.length; i++) {
      if (!common.includes(newParents[i])) added.push(newParents[i])
    }
    if (added.length) fdata.set("add-parents", added.join(" "))
    if (removed.length) fdata.set("remove-parents", removed.join(" "))

    let res = await fetch(`/${$page.data.page.slug}/parent-set`, {
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
      showParentAction = false
      invalidateAll()
    }
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

  async function handleVote() {
    let fdata = new FormData()
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    let res = await fetch(`/${$page.data.page.slug}/score`, {
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
      voteRating = res.score ?? 0
      showVote = true
    }
  }
  async function getVoteList() {
    let fdata = new FormData()
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    fdata.set("action", "get_list")
    let res = await fetch(`/${$page.data.page.slug}/vote`, {
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
      voteMap = new Map()
      res.forEach((vote) => {
        voteMap.set(vote.user_id, vote)
      })
    }
  }
  async function castVote(value?: number) {
    let fdata = new FormData()
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    fdata.set("action", "set")
    fdata.set("value", value ?? 0)
    let res = await fetch(`/${$page.data.page.slug}/vote`, {
      method: "POST",
      body: fdata
    }).then((res) => res.json())
    if (res?.message) {
      showErrorPopup.set({
        state: true,
        message: res.message,
        data: res.data
      })
    }
  }
  async function cancelVote() {
    let fdata = new FormData()
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    fdata.set("action", "remove")
    let res = await fetch(`/${$page.data.page.slug}/vote`, {
      method: "POST",
      body: fdata
    }).then((res) => res.json())
    if (res?.message) {
      showErrorPopup.set({
        state: true,
        message: res.message,
        data: res.data
      })
    }
  }

  async function getFileList(deleted?: boolean) {
    let fdata = new FormData()
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    if (deleted !== undefined) fdata.set("deleted", deleted)
    let res = await fetch(`/${$page.data.page.slug}/file-list`, {
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
      fileMap = new Map()
      res.forEach((file) => {
        fileMap.set(file.file_id, file)
      })
    }
  }

  async function uploadFile() {
    let form = document.getElementById("file-upload")
    let fdata = new FormData(form)
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)

    let res = await fetch(`/${$page.data.page.slug}/file-upload`, {
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
      filesUpload = null
      showFileUploadAction = false
      await getFileList()
    }
  }

  async function deleteFile(fileId: number, lastRevisionId: number) {
    let fdata = new FormData()
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    fdata.set("file-id", fileId)
    fdata.set("last-revision-id", lastRevisionId)

    let res = await fetch(`/${$page.data.page.slug}/file-delete`, {
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
      showFileHistory = false
      await getFileList()
    }
  }

  async function editFile() {
    let form = document.getElementById("file-edit")
    let fdata = new FormData(form)
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    fdata.set("file-id", fileEditId)
    fdata.set("last-revision-id", fileMap.get(fileEditId)?.revision_id)

    if (!filesEditElem.files?.length) fdata.delete("file")

    let res = await fetch(`/${$page.data.page.slug}/file-edit`, {
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
      showFileEditAction = false
      showFileHistory = false
      await getFileList()
    }
  }

  async function moveFile() {
    let form = document.getElementById("file-move")
    let fdata = new FormData(form)
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    fdata.set("file-id", fileEditId)
    fdata.set("last-revision-id", fileMap.get(fileEditId)?.revision_id)

    let res = await fetch(`/${$page.data.page.slug}/file-move`, {
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
      showFileMoveAction = false
      showFileHistory = false
      await getFileList()
    }
  }

  async function restoreFile() {
    let form = document.getElementById("file-restore")
    let fdata = new FormData(form)
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    fdata.set("file-id", fileEditId)
    let res = await fetch(`/${$page.params.slug}/file-restore`, {
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
      showFileRestoreAction = false
      getFileList()
      invalidateAll()
    }
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
  <form id="editor" class="editor" method="POST" on:submit|preventDefault={saveEdit}>
    <input
      name="title"
      class="editor-title"
      placeholder={$page.data.internationalization?.title}
      type="text"
      value={$page.data.page_revision.title}
    />
    <input
      name="alt-title"
      class="editor-alt-title"
      placeholder={$page.data.internationalization?.["alt-title"]}
      type="text"
      value={$page.data.page_revision.alt_title}
    />
    <textarea name="wikitext" class="editor-wikitext">{$page.data.wikitext}</textarea>
    <input
      name="tags"
      class="editor-tags"
      placeholder={$page.data.internationalization?.tags}
      type="text"
      value={$page.data.page_revision.tags.join(" ")}
    />
    <textarea
      name="comments"
      class="editor-comments"
      placeholder={$page.data.internationalization?.["wiki-page-revision-comments"]}
    />
    <div class="action-row editor-actions">
      <button
        class="action-button editor-button button-cancel clickable"
        type="button"
        on:click|stopPropagation={cancelEdit}
      >
        {$page.data.internationalization?.cancel}
      </button>
      <button
        class="action-button editor-button button-save clickable"
        type="submit"
        on:click|stopPropagation
      >
        {$page.data.internationalization?.save}
      </button>
    </div>
  </form>
{:else}
  <div class="action-row editor-actions">
    <button
      class="action-button editor-button button-move clickable"
      type="button"
      on:click={() => {
        showMoveAction = true
      }}
    >
      {$page.data.internationalization?.move}
    </button>
    <button
      class="action-button editor-button button-layout clickable"
      type="button"
      on:click={() => {
        showLayoutAction = true
      }}
    >
      {$page.data.internationalization?.layout}
    </button>
    <button
      class="action-button editor-button button-parents clickable"
      type="button"
      on:click={getParents}
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
      on:click={handleVote}
    >
      {$page.data.internationalization?.vote}
    </button>
    <button
      class="action-button button-files clickable"
      type="button"
      on:click={() => {
        getFileList().then(() => {
          showFiles = true
        })
      }}
    >
      {$page.data.internationalization?.files}
    </button>
  </div>
{/if}

{#if showSource}
  <textarea class="page-source" readonly={true}>{$page.data.wikitext}</textarea>
{/if}

{#if showMoveAction}
  <form
    id="page-move"
    class="page-move"
    method="POST"
    on:submit|preventDefault={handleMove}
  >
    <input
      bind:this={moveInputNewSlugElem}
      name="new-slug"
      class="page-move-new-slug"
      placeholder={$page.data.internationalization?.["wiki-page-move-new-slug"]}
      type="text"
    />
    <textarea
      name="comments"
      class="page-move-comments"
      placeholder={$page.data.internationalization?.["wiki-page-revision-comments"]}
    />
    <div class="action-row page-move-actions">
      <button
        class="action-button page-move-button button-cancel clickable"
        type="button"
        on:click|stopPropagation={() => {
          showMoveAction = false
        }}
      >
        {$page.data.internationalization?.cancel}
      </button>
      <button
        class="action-button page-move-button button-move clickable"
        type="submit"
        on:click|stopPropagation
      >
        {$page.data.internationalization?.move}
      </button>
    </div>
  </form>
{/if}

{#if showLayoutAction}
  <form
    id="page-layout"
    class="page-layout"
    method="POST"
    on:submit|preventDefault={handleLayout}
  >
    <select name="layout" class="page-layout-select" value={$page.data.page.layout}>
      <option value={null}
        >{$page.data.internationalization?.["wiki-page-layout.default"]}</option
      >
      {#each Object.values(Layout) as layoutOption}
        <option value={layoutOption}
          >{$page.data.internationalization?.[`wiki-page-layout.${layoutOption}`]}</option
        >
      {/each}
    </select>
    <div class="action-row page-layout-actions">
      <button
        class="action-button page-layout-button button-cancel clickable"
        type="button"
        on:click|stopPropagation={() => {
          showLayoutAction = false
        }}
      >
        {$page.data.internationalization?.cancel}
      </button>
      <button
        class="action-button page-layout-button button-save clickable"
        type="submit"
        on:click|stopPropagation
      >
        {$page.data.internationalization?.save}
      </button>
    </div>
  </form>
{/if}

{#if showParentAction}
  <form
    id="page-parent"
    class="page-parent"
    method="POST"
    on:submit|preventDefault={setParents}
  >
    <input
      name="parents"
      class="page-parent-new-parents"
      placeholder={$page.data.internationalization?.parents}
      type="text"
      value={parents}
    />
    <div class="action-row page-parent-actions">
      <button
        class="action-button page-parent-button button-cancel clickable"
        type="button"
        on:click|stopPropagation={() => {
          showParentAction = false
        }}
      >
        {$page.data.internationalization?.cancel}
      </button>
      <button
        class="action-button page-parent-button button-save clickable"
        type="submit"
        on:click|stopPropagation
      >
        {$page.data.internationalization?.save}
      </button>
    </div>
  </form>
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

{#if showVote}
  <div class="vote-panel">
    <div class="action-row vote-action">
      <button
        class="action-button view-vote-list clickable"
        type="button"
        on:click|stopPropagation={() => {
          getVoteList().then(() => {
            showVoteList = true
          })
        }}
      >
        {$page.data.internationalization?.["wiki-page-vote-list"]}
      </button>
      <div class="action-button vote-rating">
        <span class="vote-desc"
          >{$page.data.internationalization?.["wiki-page-vote-score"]}</span
        >
        <span class="vote-rating-number">{voteRating}</span>
      </div>
      <div class="action-button cast-vote">
        <span class="vote-desc"
          >{$page.data.internationalization?.["wiki-page-vote-set"]}</span
        >
        <button
          class="vote-subbutton clickable"
          type="button"
          on:click|stopPropagation={() => castVote(1)}
        >
          +1
        </button>
        <button
          class="vote-subbutton clickable"
          type="button"
          on:click|stopPropagation={() => castVote(0)}
        >
          0
        </button>
        <button
          class="vote-subbutton clickable"
          type="button"
          on:click|stopPropagation={() => castVote(-1)}
        >
          -1
        </button>
      </div>
      <button
        class="action-button remove-vote clickable"
        type="button"
        on:click|stopPropagation={cancelVote}
      >
        {$page.data.internationalization?.["wiki-page-vote-remove"]}
      </button>
    </div>
    {#if showVoteList}
      <ul class="vote-list">
        {#each [...voteMap].sort((a, b) => b[0] - a[0]) as [_, vote] (vote.page_vote_id)}
          <li class="vote-item" data-id={vote.page_vote_id} data-user-id={vote.user_id}>
            UT: User {vote.user_id}: {vote.value}
          </li>
        {/each}
      </ul>
    {/if}
  </div>
{/if}

{#if showFiles}
  <div class="file-panel">
    <div class="action-row file-action">
      <button
        class="action-button upload-file clickable"
        type="button"
        on:click|stopPropagation={() => {
          showFileUploadAction = true
        }}
      >
        {$page.data.internationalization?.upload}
      </button>
      <button
        class="action-button deleted-file clickable"
        type="button"
        on:click={() => {
          showFiles = false
          getFileList(true).then(() => {
            showFiles = true
          })
        }}
      >
        {$page.data.internationalization?.restore}
      </button>
    </div>

    {#if fileMap.size > 0}
      <div class="file-list">
        <div class="file-list-header">
          <div class="file-attribute name">
            {$page.data.internationalization?.["wiki-page-file.name"]}
          </div>
          <div class="file-attribute created-at">
            {$page.data.internationalization?.["wiki-page-file.created-at"]}
          </div>
          <div class="file-attribute updated-at">
            {$page.data.internationalization?.["wiki-page-file.updated-at"]}
          </div>
          <div class="file-attribute licensing">
            {$page.data.internationalization?.["wiki-page-file.license"]}
          </div>
          <div class="file-attribute mime">
            {$page.data.internationalization?.["wiki-page-file.mime"]}
          </div>
          <div class="file-attribute size">
            {$page.data.internationalization?.["wiki-page-file.size"]}
          </div>
          <div class="file-attribute action" />
        </div>
        {#each [...fileMap].sort((a, b) => b[0] - a[0]) as [_, file] (file.file_id)}
          <div class="file-row" data-id={file.file_id}>
            <div class="file-attribute name">
              {file.name}
            </div>
            <div class="file-attribute created-at">
              {new Date(file.file_created_at).toLocaleString()}
            </div>
            <div class="file-attribute updated-at">
              {file.file_updated_at
                ? new Date(file.file_updated_at).toLocaleString()
                : ""}
            </div>
            <div class="file-attribute licensing">
              {file.licensing}
            </div>
            <div class="file-attribute mime">
              {file.mime.split(";")[0]}
            </div>
            <div class="file-attribute size">
              {file.size}
            </div>
            <div class="file-attribute action">
              {#if file.revision_type === "delete"}
                <button
                  class="action-button restore-file clickable"
                  type="button"
                  on:click|stopPropagation={() => {
                    fileEditId = file.file_id
                    showFileRestoreAction = true
                  }}
                >
                  {$page.data.internationalization?.restore}
                </button>
              {:else}
                <button
                  class="action-button move-file clickable"
                  type="button"
                  on:click|stopPropagation={() => {
                    fileEditId = file.file_id
                    showFileMoveAction = true
                  }}
                >
                  {$page.data.internationalization?.move}
                </button>
                <button
                  class="action-button edit-file clickable"
                  type="button"
                  on:click|stopPropagation={() => {
                    fileEditId = file.file_id
                    showFileEditAction = true
                  }}
                >
                  {$page.data.internationalization?.edit}
                </button>
                <button
                  class="action-button delete-file clickable"
                  type="button"
                  on:click|stopPropagation={() => {
                    deleteFile(file.file_id, file.revision_id)
                  }}
                >
                  {$page.data.internationalization?.delete}
                </button>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    {:else}
      <div class="file-list">
        <div class="file-list-message">
          {$page.data.internationalization?.["wiki-page-file-no-files"]}
        </div>
      </div>
    {/if}

    {#if showFileUploadAction}
      <form
        id="file-upload"
        class="file-upload"
        method="POST"
        on:submit|preventDefault={uploadFile}
      >
        <div class="file-form-field">
          <label for="file"
            >{$page.data.internationalization?.["wiki-page-file-select"]}</label
          >
          <input
            name="file"
            class="file-attribute file"
            type="file"
            bind:files={filesUpload}
          />
        </div>
        <div class="file-form-field">
          <label for="name"
            >{$page.data.internationalization?.["wiki-page-file-name"]}</label
          >
          <input
            name="name"
            class="file-attribute name"
            placeholder={filesUpload?.[0]?.name}
            type="text"
          />
        </div>
        <textarea
          name="comments"
          class="file-form-field file-comments"
          placeholder={$page.data.internationalization?.["wiki-page-revision-comments"]}
        />
        <div class="action-row file-upload-actions">
          <button
            class="action-button file-upload-button button-cancel clickable"
            type="button"
            on:click|stopPropagation={() => {
              filesUpload = null
              showFileUploadAction = false
            }}
          >
            {$page.data.internationalization?.cancel}
          </button>
          <button
            class="action-button file-upload-button button-upload clickable"
            type="submit"
            on:click|stopPropagation
          >
            {$page.data.internationalization?.upload}
          </button>
        </div>
      </form>
    {/if}

    {#if showFileEditAction}
      <form
        id="file-edit"
        class="file-edit"
        method="POST"
        on:submit|preventDefault={editFile}
      >
        <div class="file-form-field">
          <label for="file"
            >{$page.data.internationalization?.["wiki-page-file-select"]}</label
          >
          <input
            bind:this={filesEditElem}
            name="file"
            class="file-attribute file"
            type="file"
          />
        </div>
        <div class="file-form-field">
          <label for="name"
            >{$page.data.internationalization?.["wiki-page-file-name"]}</label
          >
          <input
            name="name"
            class="file-attribute name"
            placeholder={fileMap.get(fileEditId)?.name}
            type="text"
          />
        </div>
        <textarea
          name="comments"
          class="file-form-field file-comments"
          placeholder={$page.data.internationalization?.["wiki-page-revision-comments"]}
        />
        <div class="action-row file-edit-actions">
          <button
            class="action-button file-edit-button button-cancel clickable"
            type="button"
            on:click|stopPropagation={() => {
              showFileEditAction = false
            }}
          >
            {$page.data.internationalization?.cancel}
          </button>
          <button
            class="action-button file-edit-button button-save clickable"
            type="submit"
            on:click|stopPropagation
          >
            {$page.data.internationalization?.save}
          </button>
        </div>
      </form>
    {/if}

    {#if showFileMoveAction}
      <form
        id="file-move"
        class="file-move"
        method="POST"
        on:submit|preventDefault={moveFile}
      >
        <input
          name="destination-page"
          class="file-move-destination-page"
          placeholder={$page.data.internationalization?.["wiki-page-file-move-destination-page"]}
          type="text"
        />
        <textarea
          name="comments"
          class="file-move-comments"
          placeholder={$page.data.internationalization?.["wiki-page-revision-comments"]}
        />
        <div class="action-row file-move-actions">
          <button
            class="action-button file-move-button button-cancel clickable"
            type="button"
            on:click|stopPropagation={() => {
              showFileMoveAction = false
            }}
          >
            {$page.data.internationalization?.cancel}
          </button>
          <button
            class="action-button file-move-button button-move clickable"
            type="submit"
            on:click|stopPropagation
          >
            {$page.data.internationalization?.move}
          </button>
        </div>
      </form>
    {/if}

    {#if showFileRestoreAction}
      <form
        id="file-restore"
        class="file-restore"
        method="POST"
        on:submit|preventDefault={restoreFile}
      >
        <input
          name="new-page"
          class="file-restore-new-page"
          placeholder={$page.data.internationalization?.[
            "wiki-page-file-restore.new-page"
          ]}
          type="text"
        />
        <input
          name="new-name"
          class="file-restore-new-name"
          placeholder={$page.data.internationalization?.[
            "wiki-page-file-restore.new-name"
          ]}
          type="text"
        />
        <textarea
          name="comments"
          class="file-restore-comments"
          placeholder={$page.data.internationalization?.["wiki-page-revision-comments"]}
        />
        <div class="action-row file-restore-actions">
          <button
            class="action-button file-restore-button button-cancel clickable"
            type="button"
            on:click|stopPropagation={() => {
              showFileRestoreAction = false
            }}
          >
            {$page.data.internationalization?.cancel}
          </button>
          <button
            class="action-button file-restore-button button-restore clickable"
            type="submit"
            on:click|stopPropagation
          >
            {$page.data.internationalization?.restore}
          </button>
        </div>
      </form>
    {/if}
  </div>
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
  .other-actions,
  .page-move {
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

  .editor,
  .page-move,
  .page-layout,
  .page-parent,
  .file-upload,
  .file-edit,
  .file-move,
  .file-restore {
    display: flex;
    flex-direction: column;
    gap: 15px;
    align-items: stretch;
    justify-content: stretch;
    width: 80vw;
  }

  .editor-wikitext {
    height: 60vh;
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

  .file-list {
    display: table;
    width: 100%;

    .file-list-header,
    .file-row {
      display: table-row;

      .file-attribute {
        display: table-cell;
      }
    }
  }
</style>
