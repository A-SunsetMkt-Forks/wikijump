<?php

namespace Wikidot\DB;




use Ozone\Framework\Database\BaseDBPeer;

/**
 * Base peer Class mapped to the database table forum_post_revision.
 */
class ForumPostRevisionPeerBase extends BaseDBPeer
{
    public static $peerInstance;

    protected function internalInit()
    {
        $this->tableName='forum_post_revision';
        $this->objectName=ForumPostRevision::class;
        $this->primaryKeyName = 'revision_id';
        $this->fieldNames = array( 'revision_id' ,  'post_id' ,  'user_id' ,  'user_string' ,  'text' ,  'title' ,  'date' );
        $this->fieldTypes = array( 'revision_id' => 'serial',  'post_id' => 'int',  'user_id' => 'int',  'user_string' => 'varchar(80)',  'text' => 'text',  'title' => 'varchar(256)',  'date' => 'timestamp');
        $this->defaultValues = array();
    }

    public static function instance()
    {
        if (self::$peerInstance == null) {
            $className = ForumPostRevisionPeer::class;
            self::$peerInstance = new $className();
        }
        return self::$peerInstance;
    }
}
