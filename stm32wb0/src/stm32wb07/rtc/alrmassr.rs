///Register `ALRMASSR` reader
pub type R = crate::R<ALRMASSRrs>;
///Register `ALRMASSR` writer
pub type W = crate::W<ALRMASSRrs>;
///Field `SS` reader - Sub seconds value This value is compared with the contents of the synchronous prescalers counter to determine if Alarm A is to be activated. Only bits 0 up MASKSS-1 are compared.
pub type SS_R = crate::FieldReader<u16>;
///Field `SS` writer - Sub seconds value This value is compared with the contents of the synchronous prescalers counter to determine if Alarm A is to be activated. Only bits 0 up MASKSS-1 are compared.
pub type SS_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
///Field `MASKSS` reader - Mask the most-significant bits starting at this bit 0: No comparison on sub seconds for Alarm A. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match). 1: SS\[14:1\] are dont care in Alarm A comparison. Only SS\[0\] is compared. 2: SS\[14:2\] are dont care in Alarm A comparison. Only SS\[1:0\] are compared. 3: SS\[14:3\] are dont care in Alarm A comparison. Only SS\[2:0\] are compared. ... 12: SS\[14:12\] are dont care in Alarm A comparison. SS\[11:0\] are compared. 13: SS\[14:13\] are dont care in Alarm A comparison. SS\[12:0\] are compared. 14: SS\[14\] is dont care in Alarm A comparison. SS\[13:0\] are compared. 15: All 15 SS bits are compared and must match to activate alarm. The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation.
pub type MASKSS_R = crate::FieldReader;
///Field `MASKSS` writer - Mask the most-significant bits starting at this bit 0: No comparison on sub seconds for Alarm A. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match). 1: SS\[14:1\] are dont care in Alarm A comparison. Only SS\[0\] is compared. 2: SS\[14:2\] are dont care in Alarm A comparison. Only SS\[1:0\] are compared. 3: SS\[14:3\] are dont care in Alarm A comparison. Only SS\[2:0\] are compared. ... 12: SS\[14:12\] are dont care in Alarm A comparison. SS\[11:0\] are compared. 13: SS\[14:13\] are dont care in Alarm A comparison. SS\[12:0\] are compared. 14: SS\[14\] is dont care in Alarm A comparison. SS\[13:0\] are compared. 15: All 15 SS bits are compared and must match to activate alarm. The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation.
pub type MASKSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:14 - Sub seconds value This value is compared with the contents of the synchronous prescalers counter to determine if Alarm A is to be activated. Only bits 0 up MASKSS-1 are compared.
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0x7fff) as u16)
    }
    ///Bits 24:27 - Mask the most-significant bits starting at this bit 0: No comparison on sub seconds for Alarm A. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match). 1: SS\[14:1\] are dont care in Alarm A comparison. Only SS\[0\] is compared. 2: SS\[14:2\] are dont care in Alarm A comparison. Only SS\[1:0\] are compared. 3: SS\[14:3\] are dont care in Alarm A comparison. Only SS\[2:0\] are compared. ... 12: SS\[14:12\] are dont care in Alarm A comparison. SS\[11:0\] are compared. 13: SS\[14:13\] are dont care in Alarm A comparison. SS\[12:0\] are compared. 14: SS\[14\] is dont care in Alarm A comparison. SS\[13:0\] are compared. 15: All 15 SS bits are compared and must match to activate alarm. The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation.
    #[inline(always)]
    pub fn maskss(&self) -> MASKSS_R {
        MASKSS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALRMASSR")
            .field("ss", &self.ss())
            .field("maskss", &self.maskss())
            .finish()
    }
}
impl W {
    ///Bits 0:14 - Sub seconds value This value is compared with the contents of the synchronous prescalers counter to determine if Alarm A is to be activated. Only bits 0 up MASKSS-1 are compared.
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W<'_, ALRMASSRrs> {
        SS_W::new(self, 0)
    }
    ///Bits 24:27 - Mask the most-significant bits starting at this bit 0: No comparison on sub seconds for Alarm A. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match). 1: SS\[14:1\] are dont care in Alarm A comparison. Only SS\[0\] is compared. 2: SS\[14:2\] are dont care in Alarm A comparison. Only SS\[1:0\] are compared. 3: SS\[14:3\] are dont care in Alarm A comparison. Only SS\[2:0\] are compared. ... 12: SS\[14:12\] are dont care in Alarm A comparison. SS\[11:0\] are compared. 13: SS\[14:13\] are dont care in Alarm A comparison. SS\[12:0\] are compared. 14: SS\[14\] is dont care in Alarm A comparison. SS\[13:0\] are compared. 15: All 15 SS bits are compared and must match to activate alarm. The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation.
    #[inline(always)]
    pub fn maskss(&mut self) -> MASKSS_W<'_, ALRMASSRrs> {
        MASKSS_W::new(self, 24)
    }
}
/**RTC_ALRMASSR register

You can [`read`](crate::Reg::read) this register and get [`alrmassr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmassr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RTC:ALRMASSR)*/
pub struct ALRMASSRrs;
impl crate::RegisterSpec for ALRMASSRrs {
    type Ux = u32;
}
///`read()` method returns [`alrmassr::R`](R) reader structure
impl crate::Readable for ALRMASSRrs {}
///`write(|w| ..)` method takes [`alrmassr::W`](W) writer structure
impl crate::Writable for ALRMASSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ALRMASSR to value 0
impl crate::Resettable for ALRMASSRrs {}
