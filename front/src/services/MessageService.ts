import http from '../http-common'

import { AxiosResponse } from 'axios'

import Messages from '../types/Messages'
import CreateMessage from '../types/CreateMessage'

class MessageService {
  getAll(): Promise<AxiosResponse<Messages, any>> {
    return http.get<Messages>('/msg')
  }

  create(data: CreateMessage): Promise<CreateMessage> {
    return http.post('/msg', data)
  }
}

export default new MessageService()
