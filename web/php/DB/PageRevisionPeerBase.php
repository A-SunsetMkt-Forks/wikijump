<?php

namespace Wikidot\DB;




use Ozone\Framework\Database\BaseDBPeer;

/**
 * Base peer Class mapped to the database table page_revision.
 */
class PageRevisionPeerBase extends BaseDBPeer
{
    public static $peerInstance;

    protected function internalInit()
    {
        $this->tableName='page_revision';
        $this->objectName=PageRevision::class;
        $this->primaryKeyName = 'revision_id';
        $this->fieldNames = array( 'revision_id' ,  'page_id' ,  'site_id' ,   'metadata_id' ,  'flags' ,  'flag_text' ,  'flag_title' ,  'flag_file' ,  'flag_rename' ,  'flag_meta' ,  'flag_new' ,   'revision_number' ,  'date_last_edited' ,  'user_id' ,  'user_string' ,  'comments' );
        $this->fieldTypes = array( 'revision_id' => 'serial',  'page_id' => 'int',  'site_id' => 'int',  'metadata_id' => 'int',  'flags' => 'varchar(100)',  'flag_text' => 'boolean',  'flag_title' => 'boolean',  'flag_file' => 'boolean',  'flag_rename' => 'boolean',  'flag_meta' => 'boolean',  'flag_new' => 'boolean',  'revision_number' => 'int',  'date_last_edited' => 'timestamp',  'user_id' => 'int',  'user_string' => 'varchar(80)',  'comments' => 'text');
        $this->defaultValues = array( 'flag_text' => 'false',  'flag_title' => 'false',  'flag_file' => 'false',  'flag_rename' => 'false',  'flag_meta' => 'false',  'flag_new' => 'false',  'revision_number' => '0');
    }

    public static function instance()
    {
        if (self::$peerInstance == null) {
            $className = PageRevisionPeer::class;
            self::$peerInstance = new $className();
        }
        return self::$peerInstance;
    }
}
