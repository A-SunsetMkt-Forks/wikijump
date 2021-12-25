<?php

namespace Wikidot\DB;




use Ozone\Framework\Database\BaseDBPeer;

/**
 * Base peer Class mapped to the database table page_metadata.
 */
class PageMetadataPeerBase extends BaseDBPeer
{
    public static $peerInstance;

    protected function internalInit()
    {
        $this->tableName='page_metadata';
        $this->objectName=PageMetadata::class;
        $this->primaryKeyName = 'metadata_id';
        $this->fieldNames = array( 'metadata_id' ,  'parent_page_id' ,  'title' ,  'unix_name' ,  'owner_user_id' );
        $this->fieldTypes = array( 'metadata_id' => 'serial',  'parent_page_id' => 'int',  'title' => 'varchar(256)',  'unix_name' => 'varchar(80)',  'owner_user_id' => 'int');
        $this->defaultValues = array();
    }

    public static function instance()
    {
        if (self::$peerInstance == null) {
            $className = PageMetadataPeer::class;
            self::$peerInstance = new $className();
        }
        return self::$peerInstance;
    }
}
