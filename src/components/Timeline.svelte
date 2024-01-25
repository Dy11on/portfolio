<script>
	import {
		Timeline,
		TimelineItem,
		TimelineSeparator,
		TimelineDot,
		TimelineConnector,
		TimelineContent,
		TimelineOppositeContent
	} from 'svelte-vertical-timeline';

	const options = [
		{ title: 'Software Engineer II',
    company:'Wave Financial', 
    description:'Developed front-end applications using React and back-end systems with Django, ensuring a seamless fusion of user-facing features and robust functionality.', 
    description2: 'Pioneered the implementation of a new testing library, resulting in a substantial reduction of production bugs and elevating overall software quality.', 
    description3:'Documented and established a systematic approach for generating vital test data, pivotal in facilitating the seamless migration of legacy codebases.', 
    year: 'Mar 2022 - Mar 2023' },

		{ title: 'Software Engineer',
    company:'Canadian Tire', 
    description:'Dramatically enhanced the pace of our CI/CD deployment workflow, resulting in a notable improvement of 70-80\% faster execution compared to previous benchmarks', 
    description2: 'Proficiently set up new environments by leveraging Microsoft Azure, Kubernetes, Python, and Terraform.', 
    description3:'Skillfully developed microservices using Spring Boot, and then orchestrated their deployment through GitHub Actions.', 
    year: 'Feb 2021 - Mar 2022' },

		{ title: 'Vulnerability Management Intern',
    company:'Canadian Tire', 
    description:'Conducted comprehensive data analysis on sizable files utilizing Pandas and Python, extracting essential insights utilized in reports provided to Vice Presidents.', 
    description2: 'Improved the performance of a heavily used team-wide program by 70\%', 
    description3: '',
    year: 'Sep 2020 - Dec 2020' },

		{ title: 'Security Consultant Intern',
    company:'Security Compass', 
    description:'Performed grey-box and black-box penetration tests against web and mobile applications to find multiple vulnerabilities, contributing to client retention', 
    description2: 'Communicated with clients of differing technical levels, explaining different vulnerabilities and the potential business impact', 
    description3: '',
    year: 'May 2019 - Dec 2019' },
	];
</script>

<Timeline position="right" style="border: solid 1px #7C3AED; padding: 50px 0; border-radius: 2%;">
<h1 style="text-align: center; padding-bottom: 1em ;"class=" font-semibold text-3xl sm:text-4xl md:text-5xl">Experience</h1>
  {#each options as option}
		<TimelineItem>
			<TimelineOppositeContent slot="opposite-content">
				<p style="color: white">
        {option.year}
        </p>
			</TimelineOppositeContent>
			<TimelineSeparator>
				<TimelineDot style={'background-color: #7CD5E2;'}/>
				<TimelineConnector />
			</TimelineSeparator>
			<TimelineContent>
				<h3 style="color: white">{option.title}</h3>
				<p style="color: gray">{option.company}</p>
        <br/>
				<p style="color: white">{option.description}</p>
        <br/>
				<p style="color: white">{option.description2}</p>
        <br/>
				<p style="color: white">{option.description3}</p>
			</TimelineContent>
		</TimelineItem>
  {/each}
</Timeline>

<style>
	h3 {
		letter-spacing: 1.5px;
		margin: 0;
		padding: 0;
	}
	
	p {
		letter-spacing: 1.5px;
		margin: 0;
		color: grey;
	}
</style>
