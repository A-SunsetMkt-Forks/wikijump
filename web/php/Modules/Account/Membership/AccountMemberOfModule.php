<?php

namespace Wikidot\Modules\Account\Membership;




use Ozone\Framework\Database\Criteria;
use Wikidot\DB\MemberPeer;
use Wikidot\Utils\AccountBaseModule;

class AccountMemberOfModule extends AccountBaseModule
{

    public function build($runData)
    {

        $userId = $runData->getUserId();

        // get all membership - criteria with join - wooo!
        $c = new Criteria();
        $c->add("user_id", $userId);
        $c->addJoin("site_id", "site.site_id");
        $c->add("site.deleted", false);

        $mems = MemberPeer::instance()->select($c);
        if ((is_countable($mems) ? count($mems) : 0)>0) {
            $runData->contextAdd("memberships", $mems);
        }
    }
}
