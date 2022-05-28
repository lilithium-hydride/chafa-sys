#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn symbol_print() {  // Adapted from https://hpjansson.org/chafa/ref/chafa-using.html
		unsafe {
			let pixels: [guint8; 36] = [
				0xff, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0xff,
				0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
				0xff, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0xff
			];
			let mut symbol_map: *mut ChafaSymbolMap = chafa_symbol_map_new();
			chafa_symbol_map_add_by_tags(symbol_map, ChafaSymbolTags_CHAFA_SYMBOL_TAG_ALL);

			let mut config: *mut ChafaCanvasConfig = chafa_canvas_config_new();
			chafa_canvas_config_set_geometry(config, 23, 12);
			chafa_canvas_config_set_symbol_map(config, symbol_map);

			let mut canvas: *mut ChafaCanvas = chafa_canvas_new(config);
			chafa_canvas_draw_all_pixels(
				canvas, 
				ChafaPixelType_CHAFA_PIXEL_RGBA8_UNASSOCIATED, 
				&pixels[0], 
				3, 
				3, 
				12
			);

			let gs = *chafa_canvas_build_ansi(canvas);
			let gs = String::from_raw_parts(gs.str_ as *mut i8 as *mut u8, gs.len as usize, gs.allocated_len as usize);
			
			assert_eq!(gs, "\u{1b}[0m\u{1b}[38;2;219;0;0;48;2;240;0;0m🭄\u{1b}[38;2;201;0;0;48;2;219;0;0m🭄\u{1b}[38;2;199;0;0;48;2;184;0;0m🭚\u{1b}[38;2;177;0;0;48;2;166;0;0m🭚\u{1b}[38;2;156;0;0;48;2;146;0;0m▌\u{1b}[38;2;137;0;0;48;2;127;0;0m▌\u{1b}[38;2;118;0;0;48;2;108;0;0m▌\u{1b}[38;2;86;0;0;48;2;97;0;0m🭦\u{1b}[38;2;82;0;0;48;2;70;0;0m🬿\u{1b}[38;2;65;0;0;48;2;49;0;0m🬿\u{1b}[38;2;46;0;0;48;2;27;0;0m🭑\u{1b}[38;2;11;0;0;48;2;31;0;0m▀\u{1b}[38;2;45;0;0;48;2;26;0;0m🭆\u{1b}[38;2;61;0;0;48;2;45;0;0m🭄\u{1b}[38;2;79;0;0;48;2;66;0;0m🭄\u{1b}[38;2;100;0;0;48;2;89;0;0m🭋\u{1b}[38;2;107;0;0;48;2;117;0;0m▌\u{1b}[38;2;126;0;0;48;2;136;0;0m▌\u{1b}[38;2;158;0;0;48;2;147;0;0m🭦\u{1b}[38;2;178;0;0;48;2;165;0;0m🭥\u{1b}[38;2;180;0;0;48;2;194;0;0m🬿\u{1b}[38;2;199;0;0;48;2;217;0;0m🭑\u{1b}[38;2;216;0;0;48;2;237;0;0m🭑\u{1b}[0m
\u{1b}[38;2;178;0;0;48;2;198;0;0m🭆\u{1b}[38;2;168;0;0;48;2;185;0;0m🭆\u{1b}[38;2;158;0;0;48;2;171;0;0m🭆\u{1b}[38;2;149;0;0;48;2;159;0;0m🭄\u{1b}[38;2;147;0;0;48;2;139;0;0m🭛\u{1b}[38;2;133;0;0;48;2;127;0;0m▌\u{1b}[38;2;121;0;0;48;2;115;0;0m▌\u{1b}[38;2;111;0;0;48;2;103;0;0m🬿\u{1b}[38;2;100;0;0;48;2;89;0;0m🭑\u{1b}[38;2;90;0;0;48;2;76;0;0m🭑\u{1b}[38;2;81;0;0;48;2;62;0;0m🭑\u{1b}[38;2;52;0;0;48;2;73;0;0m▀\u{1b}[38;2;80;0;0;48;2;62;0;0m🭆\u{1b}[38;2;90;0;0;48;2;75;0;0m🭆\u{1b}[38;2;99;0;0;48;2;88;0;0m🭄\u{1b}[38;2;101;0;0;48;2;109;0;0m🭚\u{1b}[38;2;115;0;0;48;2;120;0;0m▌\u{1b}[38;2;126;0;0;48;2;132;0;0m▌\u{1b}[38;2;146;0;0;48;2;138;0;0m🬉\u{1b}[38;2;159;0;0;48;2;149;0;0m🭥\u{1b}[38;2;157;0;0;48;2;171;0;0m🭑\u{1b}[38;2;167;0;0;48;2;184;0;0m🭑\u{1b}[38;2;177;0;0;48;2;197;0;0m🭑\u{1b}[0m
\u{1b}[38;2;158;0;0;48;2;138;0;0m▀\u{1b}[38;2;152;0;0;48;2;136;0;0m▀\u{1b}[38;2;146;0;0;48;2;133;0;0m▀\u{1b}[38;2;131;0;0;48;2;140;0;0m🭆\u{1b}[38;2;129;0;0;48;2;134;0;0m🭆\u{1b}[38;2;127;0;0;48;2;130;0;0m🭃\u{1b}[38;2;121;0;0;48;2;124;0;0m🭥\u{1b}[38;2;122;0;0;48;2;116;0;0m🭑\u{1b}[38;2;120;0;0;48;2;110;0;0m🭑\u{1b}[38;2;104;0;0;48;2;118;0;0m▀\u{1b}[38;2;98;0;0;48;2;116;0;0m▀\u{1b}[38;2;94;0;0;48;2;114;0;0m▀\u{1b}[38;2;98;0;0;48;2;115;0;0m▀\u{1b}[38;2;104;0;0;48;2;118;0;0m▀\u{1b}[38;2;120;0;0;48;2;110;0;0m🭆\u{1b}[38;2;122;0;0;48;2;115;0;0m🭆\u{1b}[38;2;121;0;0;48;2;124;0;0m🬄\u{1b}[38;2;130;0;0;48;2;127;0;0m🭤\u{1b}[38;2;129;0;0;48;2;134;0;0m🭑\u{1b}[38;2;131;0;0;48;2;140;0;0m🭑\u{1b}[38;2;145;0;0;48;2;133;0;0m▀\u{1b}[38;2;151;0;0;48;2;135;0;0m▀\u{1b}[38;2;157;0;0;48;2;138;0;0m▀\u{1b}[0m
\u{1b}[38;2;118;0;0;48;2;98;0;0m▀\u{1b}[38;2;119;0;0;48;2;103;0;0m▀\u{1b}[38;2;121;0;0;48;2;108;0;0m▀\u{1b}[38;2;114;0;0;48;2;122;0;0m🭑\u{1b}[38;2;119;0;0;48;2;124;0;0m🭑\u{1b}[38;2;126;0;0;48;2;125;0;0m🭮\u{1b}[38;2;130;0;0;48;2;127;0;0m🭆\u{1b}[38;2;135;0;0;48;2;129;0;0m🭆\u{1b}[38;2;130;0;0;48;2;140;0;0m▀\u{1b}[38;2;132;0;0;48;2;146;0;0m▀\u{1b}[38;2;133;0;0;48;2;151;0;0m▀\u{1b}[38;2;134;0;0;48;2;155;0;0m▀\u{1b}[38;2;133;0;0;48;2;151;0;0m▀\u{1b}[38;2;132;0;0;48;2;146;0;0m▀\u{1b}[38;2;141;0;0;48;2;130;0;0m🭑\u{1b}[38;2;135;0;0;48;2;129;0;0m🭑\u{1b}[38;2;130;0;0;48;2;127;0;0m🭑\u{1b}[38;2;127;0;0;48;2;125;0;0m▎\u{1b}[38;2;119;0;0;48;2;124;0;0m🭆\u{1b}[38;2;114;0;0;48;2;122;0;0m🭆\u{1b}[38;2;121;0;0;48;2;109;0;0m▀\u{1b}[38;2;119;0;0;48;2;103;0;0m▀\u{1b}[38;2;118;0;0;48;2;98;0;0m▀\u{1b}[0m
\u{1b}[38;2;58;0;0;48;2;78;0;0m🭑\u{1b}[38;2;70;0;0;48;2;87;0;0m🭑\u{1b}[38;2;83;0;0;48;2;96;0;0m🭑\u{1b}[38;2;95;0;0;48;2;104;0;0m🬿\u{1b}[38;2;107;0;0;48;2;114;0;0m🬓\u{1b}[38;2;120;0;0;48;2;126;0;0m▌\u{1b}[38;2;138;0;0;48;2;132;0;0m🭋\u{1b}[38;2;140;0;0;48;2;148;0;0m🭚\u{1b}[38;2;161;0;0;48;2;150;0;0m🭆\u{1b}[38;2;174;0;0;48;2;159;0;0m🭆\u{1b}[38;2;187;0;0;48;2;168;0;0m🭆\u{1b}[38;2;176;0;0;48;2;197;0;0m▀\u{1b}[38;2;187;0;0;48;2;169;0;0m🭑\u{1b}[38;2;175;0;0;48;2;160;0;0m🭑\u{1b}[38;2;163;0;0;48;2;152;0;0m🬿\u{1b}[38;2;150;0;0;48;2;142;0;0m🬿\u{1b}[38;2;138;0;0;48;2;132;0;0m🬓\u{1b}[38;2;126;0;0;48;2;120;0;0m▌\u{1b}[38;2;110;0;0;48;2;116;0;0m🭄\u{1b}[38;2;96;0;0;48;2;106;0;0m🭆\u{1b}[38;2;84;0;0;48;2;97;0;0m🭆\u{1b}[38;2;71;0;0;48;2;87;0;0m🭆\u{1b}[38;2;58;0;0;48;2;78;0;0m🭆\u{1b}[0m
\u{1b}[38;2;17;0;0;48;2;38;0;0m🭑\u{1b}[38;2;38;0;0;48;2;55;0;0m🭑\u{1b}[38;2;56;0;0;48;2;71;0;0m🬿\u{1b}[38;2;75;0;0;48;2;87;0;0m🬓\u{1b}[38;2;97;0;0;48;2;107;0;0m▌\u{1b}[38;2;116;0;0;48;2;125;0;0m▌\u{1b}[38;2;134;0;0;48;2;144;0;0m▌\u{1b}[38;2;165;0;0;48;2;154;0;0m🭋\u{1b}[38;2;184;0;0;48;2;171;0;0m🭊\u{1b}[38;2;205;0;0;48;2;189;0;0m🭊\u{1b}[38;2;223;0;0;48;2;203;0;0m🭆\u{1b}[38;2;217;0;0;48;2;238;0;0m▀\u{1b}[38;2;223;0;0;48;2;204;0;0m🭑\u{1b}[38;2;205;0;0;48;2;189;0;0m🬿\u{1b}[38;2;185;0;0;48;2;172;0;0m🬿\u{1b}[38;2;155;0;0;48;2;166;0;0m🭖\u{1b}[38;2;144;0;0;48;2;135;0;0m▌\u{1b}[38;2;126;0;0;48;2;117;0;0m▌\u{1b}[38;2;96;0;0;48;2;106;0;0m🭋\u{1b}[38;2;76;0;0;48;2;88;0;0m🬦\u{1b}[38;2;60;0;0;48;2;74;0;0m🭄\u{1b}[38;2;40;0;0;48;2;57;0;0m🭄\u{1b}[38;2;18;0;0;48;2;39;0;0m🭆\u{1b}[0m
\u{1b}[38;2;38;0;0;48;2;16;0;0m🭆\u{1b}[38;2;53;0;0;48;2;35;0;0m🭄\u{1b}[38;2;71;0;0;48;2;56;0;0m🭄\u{1b}[38;2;88;0;0;48;2;76;0;0m🭄\u{1b}[38;2;109;0;0;48;2;99;0;0m🭋\u{1b}[38;2;116;0;0;48;2;125;0;0m▌\u{1b}[38;2;134;0;0;48;2;144;0;0m▌\u{1b}[38;2;166;0;0;48;2;154;0;0m🭦\u{1b}[38;2;185;0;0;48;2;172;0;0m🭥\u{1b}[38;2;187;0;0;48;2;203;0;0m🭑\u{1b}[38;2;204;0;0;48;2;223;0;0m🭑\u{1b}[38;2;239;0;0;48;2;218;0;0m▀\u{1b}[38;2;206;0;0;48;2;226;0;0m🭄\u{1b}[38;2;189;0;0;48;2;205;0;0m🭄\u{1b}[38;2;172;0;0;48;2;185;0;0m🭄\u{1b}[38;2;167;0;0;48;2;155;0;0m🭛\u{1b}[38;2;145;0;0;48;2;135;0;0m▌\u{1b}[38;2;126;0;0;48;2;116;0;0m▌\u{1b}[38;2;107;0;0;48;2;98;0;0m▌\u{1b}[38;2;76;0;0;48;2;88;0;0m🭥\u{1b}[38;2;74;0;0;48;2;59;0;0m🬿\u{1b}[38;2;55;0;0;48;2;38;0;0m🭑\u{1b}[38;2;38;0;0;48;2;17;0;0m🭑\u{1b}[0m
\u{1b}[38;2;77;0;0;48;2;57;0;0m🭆\u{1b}[38;2;86;0;0;48;2;70;0;0m🭆\u{1b}[38;2;95;0;0;48;2;81;0;0m🭄\u{1b}[38;2;105;0;0;48;2;95;0;0m🭆\u{1b}[38;2;114;0;0;48;2;108;0;0m🭄\u{1b}[38;2;120;0;0;48;2;126;0;0m▌\u{1b}[38;2;138;0;0;48;2;132;0;0m🭦\u{1b}[38;2;151;0;0;48;2;142;0;0m🭥\u{1b}[48;2;162;0;0m🭑\u{1b}[38;2;160;0;0;48;2;175;0;0m🭑\u{1b}[38;2;169;0;0;48;2;188;0;0m🭑\u{1b}[38;2;197;0;0;48;2;177;0;0m▀\u{1b}[38;2;170;0;0;48;2;188;0;0m🭆\u{1b}[38;2;161;0;0;48;2;175;0;0m🭆\u{1b}[38;2;152;0;0;48;2;163;0;0m🭄\u{1b}[38;2;142;0;0;48;2;150;0;0m🭄\u{1b}[38;2;137;0;0;48;2;132;0;0m▌\u{1b}[38;2;126;0;0;48;2;120;0;0m▌\u{1b}[38;2;110;0;0;48;2;117;0;0m🭖\u{1b}[38;2;107;0;0;48;2;97;0;0m🬿\u{1b}[38;2;96;0;0;48;2;83;0;0m🭑\u{1b}[38;2;87;0;0;48;2;70;0;0m🭑\u{1b}[38;2;77;0;0;48;2;57;0;0m🭑\u{1b}[0m
\u{1b}[38;2;97;0;0;48;2;117;0;0m▀\u{1b}[38;2;102;0;0;48;2;119;0;0m▀\u{1b}[38;2;108;0;0;48;2;120;0;0m▀\u{1b}[38;2;122;0;0;48;2;113;0;0m🭆\u{1b}[38;2;124;0;0;48;2;119;0;0m🭆\u{1b}[38;2;126;0;0;48;2;125;0;0m🭮\u{1b}[38;2;128;0;0;48;2;131;0;0m🭏\u{1b}[38;2;129;0;0;48;2;135;0;0m🭑\u{1b}[38;2;141;0;0;48;2;131;0;0m▀\u{1b}[38;2;146;0;0;48;2;133;0;0m▀\u{1b}[38;2;152;0;0;48;2;134;0;0m▀\u{1b}[38;2;156;0;0;48;2;135;0;0m▀\u{1b}[38;2;152;0;0;48;2;134;0;0m▀\u{1b}[38;2;147;0;0;48;2;133;0;0m▀\u{1b}[38;2;141;0;0;48;2;131;0;0m▀\u{1b}[38;2;129;0;0;48;2;136;0;0m🭆\u{1b}[38;2;127;0;0;48;2;131;0;0m🭄\u{1b}[48;2;125;0;0m▎\u{1b}[38;2;124;0;0;48;2;119;0;0m🭑\u{1b}[38;2;122;0;0;48;2;114;0;0m🭑\u{1b}[38;2;108;0;0;48;2;120;0;0m▀\u{1b}[38;2;103;0;0;48;2;119;0;0m▀\u{1b}[38;2;97;0;0;48;2;117;0;0m▀\u{1b}[0m
\u{1b}[38;2;137;0;0;48;2;157;0;0m▀\u{1b}[38;2;135;0;0;48;2;151;0;0m▀\u{1b}[38;2;133;0;0;48;2;145;0;0m▀\u{1b}[38;2;139;0;0;48;2;131;0;0m🭑\u{1b}[38;2;134;0;0;48;2;129;0;0m🭑\u{1b}[38;2;130;0;0;48;2;127;0;0m🬾\u{1b}[38;2;122;0;0;48;2;125;0;0m🭆\u{1b}[38;2;116;0;0;48;2;123;0;0m🭆\u{1b}[38;2;111;0;0;48;2;121;0;0m🭆\u{1b}[38;2;118;0;0;48;2;105;0;0m▀\u{1b}[38;2;116;0;0;48;2;99;0;0m▀\u{1b}[38;2;115;0;0;48;2;94;0;0m▀\u{1b}[38;2;116;0;0;48;2;99;0;0m▀\u{1b}[38;2;118;0;0;48;2;104;0;0m▀\u{1b}[38;2;110;0;0;48;2;120;0;0m🭑\u{1b}[38;2;116;0;0;48;2;122;0;0m🭑\u{1b}[38;2;124;0;0;48;2;121;0;0m🭕\u{1b}[38;2;129;0;0;48;2;126;0;0m🬦\u{1b}[38;2;134;0;0;48;2;129;0;0m🭊\u{1b}[38;2;139;0;0;48;2;131;0;0m🭆\u{1b}[38;2;133;0;0;48;2;145;0;0m▀\u{1b}[38;2;135;0;0;48;2;151;0;0m▀\u{1b}[38;2;137;0;0;48;2;156;0;0m▀\u{1b}[0m
\u{1b}[38;2;197;0;0;48;2;177;0;0m🭑\u{1b}[38;2;184;0;0;48;2;167;0;0m🭑\u{1b}[38;2;171;0;0;48;2;157;0;0m🭑\u{1b}[38;2;159;0;0;48;2;149;0;0m🬿\u{1b}[38;2;146;0;0;48;2;139;0;0m🬓\u{1b}[38;2;132;0;0;48;2;127;0;0m▌\u{1b}[38;2;121;0;0;48;2;115;0;0m▌\u{1b}[38;2;111;0;0;48;2;104;0;0m🭚\u{1b}[38;2;90;0;0;48;2;101;0;0m🭆\u{1b}[38;2;76;0;0;48;2;91;0;0m🭆\u{1b}[38;2;63;0;0;48;2;81;0;0m🭆\u{1b}[38;2;74;0;0;48;2;53;0;0m▀\u{1b}[38;2;63;0;0;48;2;81;0;0m🭑\u{1b}[38;2;76;0;0;48;2;90;0;0m🭑\u{1b}[38;2;88;0;0;48;2;99;0;0m🬿\u{1b}[38;2;102;0;0;48;2;110;0;0m🭑\u{1b}[38;2;115;0;0;48;2;120;0;0m▌\u{1b}[38;2;126;0;0;48;2;132;0;0m▌\u{1b}[38;2;145;0;0;48;2;138;0;0m🬦\u{1b}[38;2;158;0;0;48;2;148;0;0m🭊\u{1b}[38;2;170;0;0;48;2;157;0;0m🭆\u{1b}[38;2;183;0;0;48;2;166;0;0m🭆\u{1b}[38;2;196;0;0;48;2;176;0;0m🭆\u{1b}[0m
\u{1b}[38;2;237;0;0;48;2;216;0;0m🭑\u{1b}[38;2;219;0;0;48;2;201;0;0m🬿\u{1b}[38;2;198;0;0;48;2;183;0;0m🬿\u{1b}[38;2;166;0;0;48;2;179;0;0m🭕\u{1b}[38;2;155;0;0;48;2;146;0;0m▌\u{1b}[38;2;137;0;0;48;2;127;0;0m▌\u{1b}[38;2;118;0;0;48;2;108;0;0m▌\u{1b}[38;2;90;0;0;48;2;101;0;0m🭅\u{1b}[38;2;70;0;0;48;2;82;0;0m🭄\u{1b}[38;2;49;0;0;48;2;65;0;0m🭄\u{1b}[38;2;27;0;0;48;2;46;0;0m🭆\u{1b}[38;2;32;0;0;48;2;12;0;0m▀\u{1b}[38;2;26;0;0;48;2;46;0;0m🭑\u{1b}[38;2;47;0;0;48;2;63;0;0m🭑\u{1b}[38;2;83;0;0;48;2;70;0;0m🭥\u{1b}[38;2;100;0;0;48;2;90;0;0m🭦\u{1b}[38;2;107;0;0;48;2;117;0;0m▌\u{1b}[38;2;126;0;0;48;2;136;0;0m▌\u{1b}[38;2;157;0;0;48;2;147;0;0m🭋\u{1b}[38;2;177;0;0;48;2;165;0;0m🬦\u{1b}[38;2;194;0;0;48;2;180;0;0m🭄\u{1b}[38;2;216;0;0;48;2;198;0;0m🭆\u{1b}[38;2;237;0;0;48;2;216;0;0m🭆\u{1b}[0m");
		}
	}
}