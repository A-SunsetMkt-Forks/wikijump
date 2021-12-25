<?php

namespace Ozone\Framework\Database;



 /* PostgreSQL database query result.
 *
 */
class PgResult implements DatabaseResult{

	public function __construct(private $result)
	{
	}

	public function nextRow() {
		return pg_fetch_assoc($this->result);
	}

	public function resetPosition() {

	}

	public function asObjects($className, $prefetched = null) {
		$out = array();
		// check if has a primary key and make it the array key tooi
		$className = str_replace('_','\\',$className);
		$peerClassName = $className.'Peer';
		$peer = new $peerClassName;
		$pkn = $peer->getPrimaryKeyName();
		while($line = pg_fetch_assoc($this->result)){
			$obj = new $className($line, $prefetched);
			$obj->setNew(false);
			if($pkn == null){
				$out[] = $obj;
			} else {
				$out[$obj->getFieldValue($pkn)] = $obj;
			}
		}
		return $out;
	}

	public function getSize(){

	}

	public function getResult(){
		return $this->result;
	}

	public function fetchAll(){
		return pg_fetch_all($this->result);
	}

}
