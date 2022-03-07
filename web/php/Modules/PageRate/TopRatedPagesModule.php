<?php

namespace Wikidot\Modules\PageRate;

use Ozone\Framework\Database\Criteria;
use Wikidot\DB\ForumThreadPeer;
use Wikidot\Utils\CacheableModule2;
use Wikidot\Utils\ProcessException;
use Wikijump\Services\Deepwell\Models\Category;
use Wikijump\Services\Deepwell\Models\Page;

class TopRatedPagesModule extends CacheableModule2
{

    protected $keyBase = 'top_rated_pages';
    protected $timeOut = 120;
    protected $delay = 0;

    public function build($runData)
    {

        $site = $runData->getTemp("site");

        $pl = $runData->getParameterList();
        $limit =  $pl->getParameterValue("limit", "MODULE");

        if ($limit === null|| !is_numeric($limit) || $limit<1 || $limit>300) {
            $limit = 10;
        }

        $order =$pl->getParameterValue("order");

        $minRating =$pl->getParameterValue("minRating");

        if ($minRating !== null && !is_numeric($minRating)) {
            $minRating = null;
        }

        $maxRating =$pl->getParameterValue("maxRating");

        if ($maxRating !== null && !is_numeric($maxRating)) {
            $maxRating = null;
        }

        $showComments = $pl->getParameterValue("comments", "MODULE");
        $categoryName = $pl->getParameterValue("category", "MODULE", "AMODULE");
        $category = null;
        if ($categoryName !== null) {
            $category = Category::findSlug($site->getSiteId(), $categoryName);
            if ($category === null) {
                throw new ProcessException(_("The category cannot be found."));
            }
        }

        $c = new Criteria();
        if ($category !== null) {
            $c->add("category_id", $category->getCategoryId());
        }
        $c->add("site_id", $site->getSiteId());

        if ($minRating !== null) {
            $c->add("rate", $minRating, '>=');
        }

        if ($maxRating !== null) {
            $c->add("rate", $maxRating, '<=');
        }

        switch ($order) {
            case 'date-created-asc':
                $c->addOrderAscending("date_created");
                break;
            case 'date-created-desc':
                $c->addOrderDescending("date_created");
                break;
            case 'rate-asc':
            case 'rating-asc':
                $c->addOrderAscending("rate");
                break;
            default:
                $c->addOrderDescending("rate");
                break;
        }

        $c->addOrderAscending("COALESCE(title, unix_name)");
        if ($limit) {
            $c->setLimit($limit);
        }

        $pages = [null]; // TODO run query

        if ($showComments) {
            foreach ($pages as &$page) {
                if ($page->getThreadId()) {
                    $thread = ForumThreadPeer::instance()->selectByPrimaryKey($page->getThreadId());
                    $noc = $thread->getNumberPosts();
                } else {
                    $noc = 0;
                }
                $page->setTemp("numberComments", $noc);
            }
        }

        $runData->contextAdd("pages", $pages);
    }
}
