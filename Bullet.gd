extends Node2D

func _on_Area2D_area_entered(area):
	if(area.get_collision_layer_bit(2)):
		queue_free()

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	move_local_x(delta * 400)
