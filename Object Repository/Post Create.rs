<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Post Create</name>
   <tag></tag>
   <elementGuidId>3188d47f-6fcf-47d5-82c9-8cd52bf3f4e4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;${create}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>5ced710b-2605-4bdf-b9d7-8d743db076b7</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}api/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.baseUrl</defaultValue>
      <description></description>
      <id>a1ac92bb-9083-4cdf-8d05-12c976ddbb86</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.create</defaultValue>
      <description></description>
      <id>f93d3d9f-6490-4730-80aa-4589073f2eff</id>
      <masked>false</masked>
      <name>create</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d4fea9f8-6abc-4efa-9c7d-87d1ba80e9ae</id>
      <masked>false</masked>
      <name>variable_1</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d6a6b48c-64c7-4ad4-84cf-cd133005e750</id>
      <masked>false</masked>
      <name>variable_2</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>2fdf7802-6007-4b8b-aaba-339ee4377ea2</id>
      <masked>false</masked>
      <name>variable_3</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5577bde4-e222-4c84-83cf-aeb3d527e200</id>
      <masked>false</masked>
      <name>variable_4</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>6f2a68ab-3848-40eb-84c5-06ef36b2ffe9</id>
      <masked>false</masked>
      <name>variable_5</name>
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
