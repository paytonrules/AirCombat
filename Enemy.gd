extends Node2D

var sprite = null
const sprites = ["enemy1_blue.png", "enemy1_green.png",
"enemy1_red.png", "enemy1_yellow.png", "enemy2_blue.png",
"enemy2_pink.png", "enemy2_red.png", "enemy2_yello.png"]

var speed = 100;

onready var explode = preload("res://Explosion.tscn").instance()

func _ready():
	speed = speed + (rustGameState.current_stage() * 10)
	
func _enter_tree():
	sprite = Sprite.new()
	var spriteName ="res://assets/graphics/enemies/" + sprites[randi() % sprites.size()] 
	sprite.texture = load(spriteName)
	add_child(sprite)

func _process(delta):
	move_local_x(-delta*speed)
	
func _on_Area2D_area_entered(area):
	if(area.get_collision_layer_bit(3)):
		explode.set_position(self.get_position())
		get_parent().add_child(explode)
		rustGameState.increment_kills()
		queue_free()
