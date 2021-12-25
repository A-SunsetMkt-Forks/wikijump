<?php

namespace Wikidot\DB;




use Ozone\Framework\Database\BaseDBPeer;

/**
 * Base peer Class mapped to the database table ozone_permission.
 */
class OzonePermissionPeerBase extends BaseDBPeer
{
    public static $peerInstance;

    protected function internalInit()
    {
        $this->tableName='ozone_permission';
        $this->objectName=OzonePermission::class;
        $this->primaryKeyName = 'permission_id';
        $this->fieldNames = array( 'permission_id' ,  'name' ,  'description' );
        $this->fieldTypes = array( 'permission_id' => 'serial',  'name' => 'varchar(50)',  'description' => 'text');
        $this->defaultValues = array();
    }

    public static function instance()
    {
        if (self::$peerInstance == null) {
            $className = OzonePermissionPeer::class;
            self::$peerInstance = new $className();
        }
        return self::$peerInstance;
    }
}
