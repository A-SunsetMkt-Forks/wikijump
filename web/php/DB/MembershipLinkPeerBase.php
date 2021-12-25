<?php

namespace Wikidot\DB;




use Ozone\Framework\Database\BaseDBPeer;

/**
 * Base peer Class mapped to the database table membership_link.
 */
class MembershipLinkPeerBase extends BaseDBPeer
{
    public static $peerInstance;

    protected function internalInit()
    {
        $this->tableName='membership_link';
        $this->objectName=MembershipLink::class;
        $this->primaryKeyName = 'link_id';
        $this->fieldNames = array( 'link_id' ,  'site_id' ,  'by_user_id' ,  'user_id' ,  'date' ,  'type' );
        $this->fieldTypes = array( 'link_id' => 'serial',  'site_id' => 'int',  'by_user_id' => 'int',  'user_id' => 'int',  'date' => 'timestamp',  'type' => 'varchar(20)');
        $this->defaultValues = array();
    }

    public static function instance()
    {
        if (self::$peerInstance == null) {
            $className = MembershipLinkPeer::class;
            self::$peerInstance = new $className();
        }
        return self::$peerInstance;
    }
}
