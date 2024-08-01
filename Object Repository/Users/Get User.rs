<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get User</name>
   <tag></tag>
   <elementGuidId>5453ada3-2e7e-4f92-b89f-8fc14f73de60</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>*/*</value>
      <webElementGuid>a1d7281b-1b3a-49f4-a53a-a7e776a02243</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${baseUrl}/api/v1/Users/${userId}</restUrl>
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
      <id>611edb94-c4cb-4b1c-ba27-aacfda35b228</id>
      <masked>false</masked>
      <name>baseUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userId</defaultValue>
      <description></description>
      <id>61b6e94d-d9b7-4324-8157-b71bec5f0751</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


WS.verifyElementPropertyValue(response, 'id', '2')


String jsonPass =
&quot;&quot;&quot;
{
  &quot;\$schema&quot;: &quot;http://json-schema.org/draft-07/schema#&quot;,
  &quot;title&quot;: &quot;Generated schema for Get User&quot;,
  &quot;type&quot;: &quot;object&quot;,
  &quot;properties&quot;: {
    &quot;id&quot;: {
      &quot;type&quot;: &quot;number&quot;
    },
    &quot;userName&quot;: {
      &quot;type&quot;: &quot;string&quot;
    },
    &quot;password&quot;: {
      &quot;type&quot;: &quot;string&quot;
    }
  },
  &quot;required&quot;: [
    &quot;id&quot;,
    &quot;userName&quot;,
    &quot;password&quot;
  ]
}
&quot;&quot;&quot;
boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
