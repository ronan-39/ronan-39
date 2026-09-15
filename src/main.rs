use leptos::prelude::*;
use leptos_meta::Title;

#[component]
fn App() -> impl IntoView {
	view! {
		<Title text="Ronan John"/>
		<div class="max-w-190 flex flex-col mx-auto w-11/12 h-full rounded-md p-2 space-y-10 mb-20">
			<div class="flex m-y-20"/>
			<Splash/>
			<About/>
			<Education/>
			<Experience/>
			<Research/>
			<Teaching/>
		</div>
	}
}


#[component]
fn Splash() -> impl IntoView {
	view! {
		<div>
			<div class="text-4xl font-medium font-display">"Ronan John"</div> 
			<p>"Ph.D. Student, Rutgers University"</p>
			<div class="gap-20"/>
			<div class="flex flex-row gap-2 mt-2">
				<ButtonLink title="Resume" link="myresume.pdf"/>
				<ButtonLink title="GitHub" link="https://github.com/ronan-39"/>
				<ButtonLink title="LinkedIn" link="https://www.linkedin.com/in/ronan-john/"/>
			</div>
		</div>
		
	}
}

#[component]
fn ButtonLink<'a>(
	title: &'a str,
	link: &'a str
) -> impl IntoView {
	view! {
		<a href=link 
		    class="inline-block bg-slate-800 hover:bg-slate-900 text-white font-sm py-1.5 px-4 rounded-full transition-colors duration-200 shadow-sm">
		    {title}
		</a>
	}
}

#[component]
fn About() -> impl IntoView {
	view! {
		<div>
			<div class="section-title">"About"</div>
			<hr class="mb-4 border-t-2 border-gray-300"/>
			<p>
"I am a 3rd year Ph.D. conducting research under Professor Kristin Dana. My research focus is on 3D vision, scene representations,
and socially cognizant robot navigation. I also research computer vision applications in agriculture. From 2024 to 2026 I was an
NSF Fellow on the SOCRATES NRT. Before this, I recieved a bachelors in Electrical Engineering also from Rutgers University in 2024."
			</p>
		</div>
	}
}

#[component]
fn Education() -> impl IntoView {
	view! {
		<div>
		<div class="section-title">"Education"</div>
		<hr class="mb-4 border-t-2 border-gray-300"/>
		<EducationComponent
			date="Sep 2024 - Present"
			title="<b>Ph.D. in Machine Learning</b>, Rutgers University"
			subtitle="Advisor Prof. Kristin Dana"
		/>
		<EducationComponent
			date="Sep 2019 - May 2024"
			title="<b>BS in Electrical Engineering</b>, Rutgers University"
			subtitle=r"Minor in CS; graduated with highest honors"
		/>
		</div>
	}
}

#[component]
fn EducationComponent<'a>(
	date: &'a str,
	title: &'a str,
	subtitle: &'a str
) -> impl IntoView {
	view! {
		<div class="flex flex-row my-2">
			<div class="w-1/3 text-sm text-text-muted font-semibold">{date}</div>
			<div class="flex flex-col">
				<p inner_html=title/>
				<div class="text-sm text-text-muted">{subtitle}</div>
			</div>
		</div>
	}
}

#[component]
fn Experience() -> impl IntoView {
	view! {
		<div>
		<div class="section-title">"Experience"</div>
		<hr class="mb-4 border-t-2 border-gray-300"/>
		<ExperienceComponent
			date="Mar '24 - Present"
			location="New Brunswick, NJ"
			title="Rutgers University, Graduate Research Assistant"
			desc=vec![
"NSF NRT Funded Fellow in the SOCRATES (Socially Cognizant Robotics for a Technologically Enhanced
Society) program from 2024-2026",
"Currently conducting research egocentric video and scene representations",
"Developed systems for autonomous data collection for agricultural applications and automated
phenotyping for cranberries"
			]
		/>
		<ExperienceComponent
			date="Summer '22, '23"
			location="Princeton, NJ"
			title="SRI International, Engineering Intern"
			desc=vec![
"Conducted testing of CCD and LIDAR imaging devices",
"Analyzed test data for imaging devices, including wafer yield and die performance"
			]
		/>
		</div>
	}
}

#[component]
fn ExperienceComponent<'a>(
	date: &'a str,
	location: &'a str,
	title: &'a str,
	desc: Vec<&'a str>
) -> impl IntoView {
	view! {
		<div class="flex flex-row mb-4">
			<div class="w-1/4 text-sm text-text-muted font-semibold">{date}</div>
			<div class="w-3/4 flex flex-col mx-5">
				<div>{title}</div>
				<ul class="list-disc list-inside pl-4 text-text-muted text-m">
					{desc.into_iter()
						.map(|n| view! { <li>{n}</li>})
						.collect_view()}
				</ul>
			</div>
		</div>
	}
}

#[component]
fn Research() -> impl IntoView {
	view! {
		<div>
		<div class="section-title">"Research"</div>
		<hr class="mb-4 border-t-2 border-gray-300"/>
		<ResearchComponent
			title=
"<b>R. John</b>, A. Kesari, V. DiMatteo, K. Dana. 'EgoCampus: Egocentric Pedestrian Eye Gaze Model and Dataset', <i>Arxiv preprint</i>"
			desc=vec![
"Collected a egocentric pedestrian video dataset with eye gaze information; over 30 hours of data.",
"Designed and trained a modular network for gaze-prediction; combines image features and video features for final
prediction.",
"Evaluated SOTA baselines with a novel evaluation metric to address saturation in standard metrics."
			]
		/>
		<ResearchComponent
			title=
"<b>R. John</b>, et al. 'Modeling Time-Lapse Trajectories to Characterize Cranberry Growth', <i>ICCV workshop on computer vision in plant phenotyping and agriculture (CVPPA 2025)</i>"
			desc=vec![
"Learned interpretable latent spaces by modeling plant growth",
"Accurate predictions on plant variety and growth period",
"Contributes a time-lapse image dataset with multiple varieties of cranberries"
			]
		/>
		</div>
	}
}

#[component]
fn ResearchComponent<'a>(
	title: &'a str,
	desc: Vec<&'a str>
) -> impl IntoView {
	view! {
		<div class="my-2">
		<p inner_html=title/>
		<ul class="list-disc list-inside pl-4 text-text-muted text-m">
			{desc.into_iter()
				.map(|n| view! {<li inner_html=n/>})
				.collect_view()}
		</ul>
		</div>
	}
}

#[component]
fn Teaching() -> impl IntoView {
	view! {
		<div>
		<div class="section-title">"Teaching"</div>
		<hr class="mb-4 border-t-2 border-gray-300"/>
		<TeachingComponent
			title="Teaching Assistant - Robotics & Computer Vision, Rutgers University"
			date="Fall 2026"
			desc=vec![
"Responsible for grading regular assignments and holding office hours.",
"Guest lectured a class of 50 students on the basics of 3D vision and robot kinematics."
			]
		/>
		<TeachingComponent
			title="Guest Lecturer - First year Seminar on Robotics & Society, Rutgers University"
			date="Spring 2026"
			desc=vec![
"Gave students an introductory lecture on foundation models in computer vision and their applications"
			]
		/>
		</div>
	}
}

#[component]
fn TeachingComponent<'a>(
	title: &'a str,
	date: &'a str,
	desc: Vec<&'a str>
) -> impl IntoView {
	view! {
		<div class="my-2">
		<p inner_html=title/>
		<p class="text-sm text-text-muted font-semibold">{date}</p>
		<ul class="list-disc list-inside pl-4 text-text-muted text-m">
			{desc.into_iter()
				.map(|n| view! { <li>{n}</li>})
				.collect_view()}
		</ul>
		</div>
	}
}

fn main() {
	console_error_panic_hook::set_once();
    mount_to_body(App);
}
