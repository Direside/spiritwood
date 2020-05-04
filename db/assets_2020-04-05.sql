# ************************************************************
# Sequel Pro SQL dump
# Version 4541
#
# http://www.sequelpro.com/
# https://github.com/sequelpro/sequelpro
#
# Host: 0.0.0.0 (MySQL 5.5.5-10.4.12-MariaDB-1:10.4.12+maria~bionic)
# Database: assets
# Generation Time: 2020-04-05 21:53:07 +0000
# ************************************************************


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;


# Dump of table characters
# ------------------------------------------------------------

DROP TABLE IF EXISTS `characters`;

CREATE TABLE `characters` (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) DEFAULT '',
  `skill` varchar(255) DEFAULT NULL,
  `equipment` int(11) DEFAULT NULL,
  `health` int(11) DEFAULT NULL,
  `move` int(11) DEFAULT NULL,
  `attack` int(11) DEFAULT NULL,
  `description` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

LOCK TABLES `characters` WRITE;
/*!40000 ALTER TABLE `characters` DISABLE KEYS */;

INSERT INTO `characters` (`id`, `name`, `skill`, `equipment`, `health`, `move`, `attack`, `description`)
VALUES
	(1,'Akeel','Cartographer: Draw and place one extra tile',1,3,2,3,'Oldest in a family of seven');

/*!40000 ALTER TABLE `characters` ENABLE KEYS */;
UNLOCK TABLES;

# Biomes
# ------------------------------------------------------------

DROP TABLE IF EXISTS `biomes`;

CREATE TABLE `biomes` (
	`id` int(11) unsigned not null auto_increment primary key,
    `name` varchar(255)
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

LOCK TABLES `biomes` WRITE;
/*!40000 ALTER TABLE `biomes` DISABLE KEYS */;

insert into `biomes` (`id`, `name`)
VALUES
	('1', 'None'),
	('2', 'Town'),
    ('3', 'Industrial'),
    ('4', 'Mystical'),
    ('5', 'Nature');

/*!40000 ALTER TABLE `biomes` ENABLE KEYS */;
UNLOCK TABLES;


# Tiles
# ------------------------------------------------------------

DROP TABLE IF EXISTS `tiles`;

CREATE TABLE `tiles` (
	`id` int(11) unsigned NOT NULL auto_increment primary key,
    `imageURL` varchar(255) DEFAULT NULL,
    `biomeID` int(11) unsigned not null,
    `exitMask` bit(4),
    `copies` int(11) default 1,
    CONSTRAINT biome_id
    FOREIGN KEY (`biomeID`)
        REFERENCES `biomes`(`id`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

insert into `tiles` (`id`, `imageURL`, `biomeID`, `exitMask`, `copies`)
values
	('1', 'tiles/bear_cave_corner.png', 4, 0011, 2),
    ('2', 'tiles/graveyard_straight.png', 3, 0011, 2),
    ('3', 'tiles/cross.png', 1, 1111, 4),
    ('4', 'tiles/straight.png', 1, 0101, 4),
    ('5', 'tiles/tee.png', 1, 0111, 4);

# Dump of table items
# ------------------------------------------------------------

DROP TABLE IF EXISTS `cards`;

CREATE TABLE `cards` (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) DEFAULT NULL,
  `description` varchar(255) DEFAULT NULL,
  `imageURL` varchar(255) default null,
  `type` ENUM('key item', 'consumable', 'equipment', 'event'),
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

LOCK TABLES `cards` WRITE;
/*!40000 ALTER TABLE `cards` DISABLE KEYS */;

INSERT INTO `cards` (`id`, `name`, `description`, `imageURL`, `type`)
VALUES
	(1,'Compass', '', '', 'key item'),
    (2, 'Torch', 'Lets you search twice in dark areas and pick the best card', '', 'consumable'),
    (3, 'Hot Dog', 'Gain 1 health', '', 'consumable'),
    (4, 'Roast Potato', 'Gain 2 health', '', 'consumable'),
    (5, 'Rotten Egg', 'Roll a dice. 1-3 lose 1 health, 4-6 gain 2 health', '', 'consumable'),
    (6, ' Machette', 'Chop a path through from one tile to another you normally could not access', '', 'consumable'),
    (7, 'Tent', 'Camp the night and restore all your health', '', 'consumable');

/*!40000 ALTER TABLE `cards` ENABLE KEYS */;
UNLOCK TABLES;

/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;
/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
