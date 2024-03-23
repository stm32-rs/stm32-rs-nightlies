#[doc = "Register `I3C_GETMXDSR` reader"]
pub type R = crate::R<I3C_GETMXDSRrs>;
#[doc = "Register `I3C_GETMXDSR` writer"]
pub type W = crate::W<I3C_GETMXDSRrs>;
#[doc = "Field `HOFFAS` reader - controller hand-off activity state This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates in which initial activity state the (other) current controller should expect the I3C bus after a controller-role hand-off to this controller-capable I3C, when returning the defining byte CRHDLY (0x91) to a GETMXDS CCC. This 2-bit field is used to return the CRHDLY1 byte in response to the GETCAPS CCC format 3, in order to state which is the activity state of this I3C when becoming controller after a controller-role hand-off, and consequently the time the former controller should wait before testing this I3C to be confirmed its ownership."]
pub type HOFFAS_R = crate::FieldReader;
#[doc = "Field `HOFFAS` writer - controller hand-off activity state This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates in which initial activity state the (other) current controller should expect the I3C bus after a controller-role hand-off to this controller-capable I3C, when returning the defining byte CRHDLY (0x91) to a GETMXDS CCC. This 2-bit field is used to return the CRHDLY1 byte in response to the GETCAPS CCC format 3, in order to state which is the activity state of this I3C when becoming controller after a controller-role hand-off, and consequently the time the former controller should wait before testing this I3C to be confirmed its ownership."]
pub type HOFFAS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FMT` reader - GETMXDS CCC format This field is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates how is returned the GETMXDS format 1 (without MaxRdTurn) and format 2 (with MaxRdTurn). This bit is used to return the 2-byte format 1 (MaxWr, MaxRd) or 5-byte format 2 (MaxWr, MaxRd, 3-byte MaxRdTurn) byte in response to the GETCAPS CCC. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=0 and LSB=RDTURN\\[7:0\\]. - Max read turnaround time is less than 256 �s. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=RDTURN\\[7:0\\]
and LSB=0. - Max read turnaround time is between 256 �s and 65535 �s - 3-byte MaxRdTurn is returned with MSB=RDTURN\\[7:0\\], middle byte=0 and LSB=0. - Max read turnaround time is between 65535 �s and 16 s."]
pub type FMT_R = crate::FieldReader;
#[doc = "Field `FMT` writer - GETMXDS CCC format This field is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates how is returned the GETMXDS format 1 (without MaxRdTurn) and format 2 (with MaxRdTurn). This bit is used to return the 2-byte format 1 (MaxWr, MaxRd) or 5-byte format 2 (MaxWr, MaxRd, 3-byte MaxRdTurn) byte in response to the GETCAPS CCC. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=0 and LSB=RDTURN\\[7:0\\]. - Max read turnaround time is less than 256 �s. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=RDTURN\\[7:0\\]
and LSB=0. - Max read turnaround time is between 256 �s and 65535 �s - 3-byte MaxRdTurn is returned with MSB=RDTURN\\[7:0\\], middle byte=0 and LSB=0. - Max read turnaround time is between 65535 �s and 16 s."]
pub type FMT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RDTURN` reader - programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and writes the value of the selected byte (via the FMT\\[1:0\\]
field) of the 3-byte MaxRdTurn which is returned in response to the GETMXDS CCC format 2 to encode the maximum read turnaround time."]
pub type RDTURN_R = crate::FieldReader;
#[doc = "Field `RDTURN` writer - programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and writes the value of the selected byte (via the FMT\\[1:0\\]
field) of the 3-byte MaxRdTurn which is returned in response to the GETMXDS CCC format 2 to encode the maximum read turnaround time."]
pub type RDTURN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TSCO` reader - clock-to-data turnaround time (tSCO) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and is used to specify the clock-to-data turnaround time tSCO (vs the value of 12 ns). This bit is used by the hardware in response to the GETMXDS CCC to return the encoded clock-to-data turnaround time via the returned MaxRd\\[5:3\\]
bits."]
pub type TSCO_R = crate::BitReader;
#[doc = "Field `TSCO` writer - clock-to-data turnaround time (tSCO) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and is used to specify the clock-to-data turnaround time tSCO (vs the value of 12 ns). This bit is used by the hardware in response to the GETMXDS CCC to return the encoded clock-to-data turnaround time via the returned MaxRd\\[5:3\\]
bits."]
pub type TSCO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - controller hand-off activity state This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates in which initial activity state the (other) current controller should expect the I3C bus after a controller-role hand-off to this controller-capable I3C, when returning the defining byte CRHDLY (0x91) to a GETMXDS CCC. This 2-bit field is used to return the CRHDLY1 byte in response to the GETCAPS CCC format 3, in order to state which is the activity state of this I3C when becoming controller after a controller-role hand-off, and consequently the time the former controller should wait before testing this I3C to be confirmed its ownership."]
    #[inline(always)]
    pub fn hoffas(&self) -> HOFFAS_R {
        HOFFAS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - GETMXDS CCC format This field is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates how is returned the GETMXDS format 1 (without MaxRdTurn) and format 2 (with MaxRdTurn). This bit is used to return the 2-byte format 1 (MaxWr, MaxRd) or 5-byte format 2 (MaxWr, MaxRd, 3-byte MaxRdTurn) byte in response to the GETCAPS CCC. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=0 and LSB=RDTURN\\[7:0\\]. - Max read turnaround time is less than 256 �s. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=RDTURN\\[7:0\\]
and LSB=0. - Max read turnaround time is between 256 �s and 65535 �s - 3-byte MaxRdTurn is returned with MSB=RDTURN\\[7:0\\], middle byte=0 and LSB=0. - Max read turnaround time is between 65535 �s and 16 s."]
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:23 - programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and writes the value of the selected byte (via the FMT\\[1:0\\]
field) of the 3-byte MaxRdTurn which is returned in response to the GETMXDS CCC format 2 to encode the maximum read turnaround time."]
    #[inline(always)]
    pub fn rdturn(&self) -> RDTURN_R {
        RDTURN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - clock-to-data turnaround time (tSCO) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and is used to specify the clock-to-data turnaround time tSCO (vs the value of 12 ns). This bit is used by the hardware in response to the GETMXDS CCC to return the encoded clock-to-data turnaround time via the returned MaxRd\\[5:3\\]
bits."]
    #[inline(always)]
    pub fn tsco(&self) -> TSCO_R {
        TSCO_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - controller hand-off activity state This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates in which initial activity state the (other) current controller should expect the I3C bus after a controller-role hand-off to this controller-capable I3C, when returning the defining byte CRHDLY (0x91) to a GETMXDS CCC. This 2-bit field is used to return the CRHDLY1 byte in response to the GETCAPS CCC format 3, in order to state which is the activity state of this I3C when becoming controller after a controller-role hand-off, and consequently the time the former controller should wait before testing this I3C to be confirmed its ownership."]
    #[inline(always)]
    #[must_use]
    pub fn hoffas(&mut self) -> HOFFAS_W<I3C_GETMXDSRrs> {
        HOFFAS_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - GETMXDS CCC format This field is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates how is returned the GETMXDS format 1 (without MaxRdTurn) and format 2 (with MaxRdTurn). This bit is used to return the 2-byte format 1 (MaxWr, MaxRd) or 5-byte format 2 (MaxWr, MaxRd, 3-byte MaxRdTurn) byte in response to the GETCAPS CCC. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=0 and LSB=RDTURN\\[7:0\\]. - Max read turnaround time is less than 256 �s. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=RDTURN\\[7:0\\]
and LSB=0. - Max read turnaround time is between 256 �s and 65535 �s - 3-byte MaxRdTurn is returned with MSB=RDTURN\\[7:0\\], middle byte=0 and LSB=0. - Max read turnaround time is between 65535 �s and 16 s."]
    #[inline(always)]
    #[must_use]
    pub fn fmt(&mut self) -> FMT_W<I3C_GETMXDSRrs> {
        FMT_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and writes the value of the selected byte (via the FMT\\[1:0\\]
field) of the 3-byte MaxRdTurn which is returned in response to the GETMXDS CCC format 2 to encode the maximum read turnaround time."]
    #[inline(always)]
    #[must_use]
    pub fn rdturn(&mut self) -> RDTURN_W<I3C_GETMXDSRrs> {
        RDTURN_W::new(self, 16)
    }
    #[doc = "Bit 24 - clock-to-data turnaround time (tSCO) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and is used to specify the clock-to-data turnaround time tSCO (vs the value of 12 ns). This bit is used by the hardware in response to the GETMXDS CCC to return the encoded clock-to-data turnaround time via the returned MaxRd\\[5:3\\]
bits."]
    #[inline(always)]
    #[must_use]
    pub fn tsco(&mut self) -> TSCO_W<I3C_GETMXDSRrs> {
        TSCO_W::new(self, 24)
    }
}
#[doc = "I3C get capability register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i3c_getmxdsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i3c_getmxdsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3C_GETMXDSRrs;
impl crate::RegisterSpec for I3C_GETMXDSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3c_getmxdsr::R`](R) reader structure"]
impl crate::Readable for I3C_GETMXDSRrs {}
#[doc = "`write(|w| ..)` method takes [`i3c_getmxdsr::W`](W) writer structure"]
impl crate::Writable for I3C_GETMXDSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I3C_GETMXDSR to value 0"]
impl crate::Resettable for I3C_GETMXDSRrs {
    const RESET_VALUE: u32 = 0;
}
