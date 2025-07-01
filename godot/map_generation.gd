extends Node2D

func _ready() -> void:
	$DungeonWalls.set_cells_terrain_connect([Vector2(1, 1),
	Vector2(1, 2),
	Vector2(2, 1),
	Vector2(2, 2)], 0, 0)
	$DungeonFloor.set_cell(Vector2(0, 0), 0, Vector2(1, 0))
	pass
