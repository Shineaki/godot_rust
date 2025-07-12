extends Node2D

const LightTexture = preload("res://Assets/Light.png")
const GRID_SIZE = 16
@onready var fog = $FOW
var fogImage = Image.new()
var fogTexture = ImageTexture.new()

var sourceLightImage = LightTexture.get_image()
var lightImage = Image.new()
var light_offset = Vector2(4, 4)

func _ready() -> void:
	fogImage = Image.create(100, 80, false, Image.FORMAT_RGBAH)
	fogImage.fill(Color.BLACK)
	lightImage = Image.create(8, 8, false, Image.FORMAT_RGBAH)
	sourceLightImage.convert(Image.FORMAT_RGBAH)

	fog.scale *= GRID_SIZE
	#lightImage = 
	fog.texture = ImageTexture.create_from_image(fogImage)
	
	$MyRustMap.generate_floors()
	$MyRustMap.generate_walls()
	var spawn_point = $MyRustMap.get_spawn_point()
	$Character.position = spawn_point
	$MyRustMap.init_shadows()
	
	var xy = spawn_point - Vector2(8, 8)
	var rounded_xy = Vector2i(round(xy.x / 16), round(xy.y / 16))
	$MyRustMap.generate_shadows(rounded_xy)
	#$MyRustMap.update_minimap(rounded_xy)
	update_fov()
	$Character.moved.connect(process_movement)

func _process(delta: float) -> void:
	print(delta)
	pass

func update_fov():
	var xy = $Character.position - Vector2(8, 8)
	var tmp_pos = Vector2i(xy.x / 16, xy.y / 16)
	for y in range(-8, 8):
		for x in range(-8, 8):
			if $MyRustMap.is_explored(tmp_pos.x + x, tmp_pos.y + y):
				fogImage.set_pixel(tmp_pos.x + x, tmp_pos.y + y, Color.from_rgba8(120, 120, 120))
			if $MyRustMap.is_visible(tmp_pos.x + x, tmp_pos.y + y):
				fogImage.set_pixel(tmp_pos.x + x, tmp_pos.y + y, Color.WHITE)
	fog.texture = ImageTexture.create_from_image(fogImage)

func process_movement():
	var xy = $Character.position - Vector2(8, 8)
	print("process_movement ", xy)
	var rounded_xy = Vector2i(round(xy.x / 16), round(xy.y / 16))
	$MyRustMap.generate_shadows(rounded_xy)
	update_fov()
	#update_fog(xy)
	#$MyRustMap.update_minimap(rounded_xy)ddddddddddddddddd
