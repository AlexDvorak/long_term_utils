use std::process::Command;
use std::collections::HashMap;
use std::str::FromStr;
fn main(){
	// collect podcast titles
	let dir = "/Users/alexbrown/Desktop/podcasts/";
	let output = Command::new("ls")
		.arg(dir)
      .output()
		.expect("failed to execute process");
	let pods = String::from_utf8_lossy(&output.stdout).into_owned();
	let podcasts: Vec<&str> = pods.split("\n").collect();
	let mut podcasts_: Vec<String> = Vec::new();
	podcasts_.pop(); // get rid of trailing newlines
	let mut podcast_dirs = Vec::new();
	for pod in &podcasts{
		podcast_dirs.push(format!("{}{}/",dir,pod));
	}
	for pod in 0..podcasts.len(){
		podcasts_.push(podcasts[pod].replace("_"," "));
	}
	for dir_counter in 0..podcast_dirs.len()-1{// for every podcast dir collect all Episodes
		let mut tags: HashMap<&str,String> = HashMap::new();
		let output = Command::new("ls")
			.arg(&podcast_dirs[dir_counter])
			.output()
			.expect("failed to execute process");
		let _podcast_files = String::from_utf8_lossy(&output.stdout).into_owned();
		let mut pod_files: Vec<&str> = _podcast_files.split("\n").collect();
		pod_files.pop();//remove trailing newlines
		for file in 0..pod_files.len(){
			let ep_data = pod_dir_to_tags(podcasts[dir_counter],pod_files[file]);
			let pod_title = format!("ALBUM={}",ep_data.pod_name);
			let ep_title = format!("TITLE={}",ep_data.ep_name);
			let ep_dir = format!("{}{}",podcast_dirs[dir_counter],pod_files[file]);
            let ep_num = formant!("TRACKNUMBER={}");
			Command::new("vorbiscomment")
				.arg("-w") // overwrite all existing tags
                .arg(ep_dir)
				.arg("-t")
				.arg(pod_title)
				.arg("-t")
				.arg(ep_title)
				.arg("-t")
				.arg(ep_data.ep_num)
				.arg("-t")
				.arg(pod_title)
				.arg(ep_dir)
				.spawn()
				.expect("No file to work on");
		}
	}
}
struct Episode{
	pod_name: String,
	ep_name: String,
	ep_num: String,
}
impl Episode{
	fn new(name:String,pod:String,num:String)->Episode{
		Episode{
			pod_name:pod,
			ep_name:name,
			ep_num:num
		}
	}
}
fn pod_dir_to_tags(pod: &str,file_: &str)->Episode{
	let mut file: Vec<&str> = file_.split("_").collect();
	file.remove(0);
	let ep_num = String::from(file[0]);
	let mut ep_name = file[1].replace("-"," ");
	ep_name.pop();
	ep_name.pop();
	ep_name.pop();
	ep_name.pop();
	return Episode::new(ep_name,String::from(pod),ep_num);
}
