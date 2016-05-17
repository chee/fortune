<?php
$fortunes = explode("%\n", file_get_contents("fortunes"));
print $fortunes[array_rand($fortunes)];
?>