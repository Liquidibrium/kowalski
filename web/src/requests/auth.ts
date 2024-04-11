export type RegisterUser = {
  email: string
  password: string
  first_name?: string
  last_name?: string
}

export async function  register(input: RegisterUser) {
  console.log('env', process.env.NEXT_PUBLIC_API_URL)
  const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/api/auth/register`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(input)
  })

  const jsonData = await response.json();

  return jsonData
}

export type LoginUser = {
  email: string
  password: string
}

export async function login(input: LoginUser) {
  const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/api/auth/login`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(input)
  })

  const jsonData = await response.json();

  return jsonData
}
