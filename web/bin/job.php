<?php

use Ozone\Framework\Ozone;
use Ozone\Framework\RunData;

chdir(dirname(__FILE__)); // unifies CLI/CGI cwd handling
require_once ('../php/setup.php');

// initialize things now

// initialize OZONE object too
Ozone::init();
$runData = new RunData();
$runData->init();
Ozone::setRunData($runData);

$jobName = $argv[1];

$classFile = WIKIJUMP_ROOT.'/php/Jobs/'.$jobName.'.php';

require_once $classFile;

$job = new $jobName();

$job->run();
