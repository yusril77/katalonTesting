<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get List User</name>
   <tag></tag>
   <elementGuidId>3cecb7f6-4aa7-4a24-ab72-9e503444af1a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${url}api/users?page=${id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.baseUrl</defaultValue>
      <description></description>
      <id>6516842e-f719-47a4-ba17-628f1ca0f3c2</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.globalId</defaultValue>
      <description></description>
      <id>2615c0a4-dbdb-428a-98d2-af5490ef3d91</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>c782aa92-d5b7-4d67-ae0d-ac5551655e5a</id>
      <masked>false</masked>
      <name>variable_1</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>11410b50-60c5-4c57-83b6-745639a83acc</id>
      <masked>false</masked>
      <name>variable_2</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>414d29cc-4c11-4ef4-a707-4f543f044046</id>
      <masked>false</masked>
      <name>variable_3</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>bdb199da-d870-431a-ae20-f8ba4422aa1e</id>
      <masked>false</masked>
      <name>variable_4</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>df8704c0-1a7f-4609-8625-b445ec2d092a</id>
      <masked>false</masked>
      <name>variable_5</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>df60c3cd-a74b-421c-a4f9-7532f976ff2f</id>
      <masked>false</masked>
      <name>variable_6</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e4103a3e-91b7-481a-ba49-c3b549970429</id>
      <masked>false</masked>
      <name>variable_7</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
