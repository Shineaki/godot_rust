extends Player

const TILE_SIZE = Vector2(16, 16)
var sprite_movement_tween: Tween
var TARGET_POSITION = Vector2(0, 0)
var moving = false

signal moved()

func _ready() -> void:
	pass

func _input(event):
	if event.is_action_pressed("Attack"):
		attack()

func attack():
	$Hands/RightHandPosition/AnimationPlayer.play("Attack")

func move(dir: Vector2):
	moving = true
	TARGET_POSITION = global_position + dir * TILE_SIZE
	sprite_movement_tween = create_tween()
	sprite_movement_tween.set_process_mode(Tween.TWEEN_PROCESS_PHYSICS)
	sprite_movement_tween.tween_property(self, "global_position", TARGET_POSITION, 0.25)
	sprite_movement_tween.set_trans(Tween.TRANS_SINE)

func _process(delta: float) -> void:
	var anim_to_play = "Idle"
	var move_direction = 0
	if moving:
		if sprite_movement_tween.is_running():
			return
		else:
			sprite_movement_tween.kill()
			moving = false
			moved.emit()
	if Input.is_action_pressed("Move_Left"):
		anim_to_play = "Run"
		move_direction = -1
		move(Vector2(-1, 0))
	elif Input.is_action_pressed("Move_Right"):
		anim_to_play = "Run"
		move_direction = 1
		move(Vector2(1, 0))
	elif Input.is_action_pressed("Move_Down"):
		anim_to_play = "Run"
		move(Vector2(0, 1))
	elif Input.is_action_pressed("Move_Up"):
		anim_to_play = "Run"
		move(Vector2(0, -1))
	elif Input.is_action_pressed("Move_UpLeft"):
		anim_to_play = "Run"
		move_direction = -1
		move(Vector2(-1, -1))
	elif Input.is_action_pressed("Move_UpRight"):
		anim_to_play = "Run"
		move_direction = 1
		move(Vector2(1, -1))
	elif Input.is_action_pressed("Move_DownLeft"):
		anim_to_play = "Run"
		move_direction = -1
		move(Vector2(-1, 1))
	elif Input.is_action_pressed("Move_DownRight"):
		anim_to_play = "Run"
		move_direction = 1
		move(Vector2(1, 1))
	
	$AnimatedSprite2D.play(anim_to_play)
	if move_direction == 0:
		return
	if $AnimatedSprite2D.flip_h != (move_direction < 0):
		$Hands.scale.x *= -1
		$AnimatedSprite2D.flip_h = (move_direction < 0)
	#var mouse_pos = get_global_mouse_position()
	#if $AnimatedSprite2D.flip_h != (mouse_pos.x < position.x):
		#$Hands.scale.x *= -1
		#$AnimatedSprite2D.flip_h = mouse_pos.x < position.x
