// <canvas class="numbers-numbers" id="numbers-numbers-naturals" width="900" height="790" data-numbers-path="_static/65536-naturals.txt"></canvas>

function NumbersXYZPlot(container) {
  var _self = this

  _self.container = container
  _self.$container = jQuery(container)

  _self.width = _self.$container.innerWidth()
  _self.height = _self.$container.innerHeight()
  _self.numberCapacity = 32000

  _self.numbers_path = _self.$container.attr("data-numbers-path")
  jQuery.get(_self.numbers_path, function(number_dump) {
    _self.processNumberDump(number_dump)

    _self.initialiseRenderer()
    _self.drawNumbers()
    _self.render()
  })
}

NumbersXYZPlot.prototype.initialiseRenderer = function initialiseRenderer() {
  var _self = this

  _self.renderer = new THREE.WebGLRenderer({antialias: true})
  _self.renderer.setClearColor(0)
  _self.renderer.setPixelRatio(window.devicePixelRatio)
  _self.renderer.setSize(_self.width, _self.height)
  _self.container.appendChild(_self.renderer.domElement)

  _self.scene = new THREE.Scene()

  var ambient_light = new THREE.AmbientLight(0xffffff)
  _self.scene.add(ambient_light)

  /*var frustumSize = 2,
      aspect = parseFloat(_self.width) / _self.height
  _self.camera = new THREE.OrthographicCamera(
    0.5 * frustumSize * aspect / - 2,
    0.5 * frustumSize * aspect / 2,
    frustumSize / 2,
    frustumSize / -2,
    0.3,
    15
  )*/

  _self.camera = new THREE.PerspectiveCamera(60, _self.width / _self.height, 1, 10000)
  _self.camera.position.set(20, 20, 20)//(2.0, 2.0, 2.0)
  _self.camera.lookAt(new THREE.Vector3(0.5, 0.5, 0.5))

  _self.controls = new THREE.OrbitControls(_self.camera, _self.renderer.domElement)
  _self.controls.enableDamping = true
  _self.controls.dampingFactor = 0.25
  _self.controls.enableZoom = true
  _self.controls.addEventListener("change", _self.render.bind(_self))

  _self.voxel_geometry = new THREE.BoxGeometry(1, 1, 1)
  _self.voxel_material = new THREE.MeshLambertMaterial({
    color: 0xffffff//000000
  })
}

NumbersXYZPlot.prototype.render = function render() {
  var _self = this

  _self.renderer.render(_self.scene, _self.camera)
}

NumbersXYZPlot.prototype.animate = function animate() {
  var _self = this

  requestAnimationFrame(animate.bind(_self))
  _self.controls.update()
  _self.render()
}

NumbersXYZPlot.prototype.processNumberDump = function processNumberDump(number_dump) {
  var _self = this

  var number_strings = number_dump.split("\n"),
      number_strings_length = number_strings.length
  _self.numbers = []
  for (var i = 0; i < _self.numberCapacity && i < number_strings_length; i++) {
    var number_str = number_strings[i]
    if (number_str.length == 0) continue
    _self.numbers.push(parseInt(number_str))
  }
  _self.maxNumber = Math.max.apply(null, _self.numbers)
  _self.numbersLength = _self.numbers.length
}

NumbersXYZPlot.prototype.drawNumbers = function drawNumbers() {
  var _self = this

  _self.xs = []
  _self.ys = []
  _self.zs = []
  for (var n = 0; n + 2 < _self.numbersLength; n++) {
    _self.xs.push(_self.numbers[n])
    _self.ys.push(_self.numbers[n+1])
    _self.zs.push(_self.numbers[n+2])
  }

  _self.xBounds = _self.getArrayBounds(_self.xs)
  _self.yBounds = _self.getArrayBounds(_self.ys)
  _self.zBounds = _self.getArrayBounds(_self.zs)

  _self.xsRescaled = _self.rescaleArray(_self.xs)
  _self.ysRescaled = _self.rescaleArray(_self.ys)
  _self.zsRescaled = _self.rescaleArray(_self.zs)

  var voxels_geometry = new THREE.Geometry()
  for (var i = 0; i < _self.numbersLength; i++) {
    var voxel = new THREE.Mesh(_self.voxel_geometry)
    //voxel.position.set(1.0 / _self.xsRescaled[i], 1.0 / _self.ysRescaled[i], 1.0 / _self.zsRescaled[i])
    voxel.position.set(_self.xsRescaled[i], _self.ysRescaled[i], _self.zsRescaled[i])
    //console.log(voxel.position)
    voxel.updateMatrix()
    voxels_geometry.merge(voxel.geometry, voxel.matrix)
  }
  var voxels_mesh = new THREE.Mesh(voxels_geometry, _self.voxel_material)
  _self.scene.add(voxels_mesh)
}

NumbersXYZPlot.prototype.getArrayBounds = function getArrayBounds(a) {
  var _self = this

  var stats = [Math.min.apply(null, a), Math.max.apply(null, a)]
  stats.push(stats[1] + stats[0]) // Midpoint
  stats.push((1.0 / stats[1]) * _self.width)
  return stats
}

NumbersXYZPlot.prototype.rescaleArray = function rescaleArray(a) {
  var _self = this

  var bounds = _self.getArrayBounds(a)
  for (var i = 0; i < a.length; i++) {
    a[i] = a[i] / bounds[1]
    /*a[i] = (a[i] - bounds[2]) * bounds[3]
    if (a[i] == 0) {
      a[i] = 1.0 / a[i]
    }*/
  }
  return a
}

jQuery(document).ready(function () {
  $(".numbers-xyz-plot").each(function () {
    window.xyz_plot = new NumbersXYZPlot(this)
  })
})
