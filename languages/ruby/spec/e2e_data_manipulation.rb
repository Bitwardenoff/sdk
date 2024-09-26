def env(name)
  ENV[name] || raise("Missing ENV['#{name}']")
end

def with_run_id(str)
  run_id = env('RUN_ID')
  "#{str}-#{run_id}"
end

def filter_projects_to_this_run(projects)
  run_id = env('RUN_ID')
  projects.filter { |p| p['name'].end_with? run_id }
end

def filter_secrets_to_this_run(secrets)
  run_id = env('RUN_ID')
  secrets.filter { |p| p['key'].end_with? run_id }
end

def project_with_run_id(project)
  project['name'] = with_run_id project['name']
  project
end

def secret_with_run_id(secret)
  secret['key'] = with_run_id secret['key']
  secret['project_name'] = with_run_id secret['project_name']
  secret
end

def secret_with_project_id(secret, projects)
  project = projects.find { |p| p['name'] == secret['project_name'] }
  secret['project_id'] = project['id']
  secret
end
