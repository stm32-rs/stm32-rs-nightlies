#[doc = "Register `AWLTR` reader"]
pub type R = crate::R<AWLTRrs>;
#[doc = "Register `AWLTR` writer"]
pub type W = crate::W<AWLTRrs>;
#[doc = "Field `BKAWL` reader - Break signal assignment to analog watchdog low threshold event BKAWL\\[i\\]
= 0: Break i signal is not assigned to an analog watchdog low threshold event BKAWL\\[i\\]
= 1: Break i signal is assigned to an analog watchdog low threshold event"]
pub type BKAWL_R = crate::FieldReader;
#[doc = "Field `BKAWL` writer - Break signal assignment to analog watchdog low threshold event BKAWL\\[i\\]
= 0: Break i signal is not assigned to an analog watchdog low threshold event BKAWL\\[i\\]
= 1: Break i signal is assigned to an analog watchdog low threshold event"]
pub type BKAWL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AWLT` reader - Analog watchdog low threshold These bits are written by software to define the low threshold for the analog watchdog. Note: In case channel transceivers monitor (AWFSEL=1), only the higher 16 bits (AWLT\\[23:8\\]) define the 16-bit threshold as compared with the analog watchdog filter output (because data coming from the analog watchdog filter are up to a 16-bit resolution). Bits AWLT\\[7:0\\]
are not taken into comparison in this case."]
pub type AWLT_R = crate::FieldReader<u32>;
#[doc = "Field `AWLT` writer - Analog watchdog low threshold These bits are written by software to define the low threshold for the analog watchdog. Note: In case channel transceivers monitor (AWFSEL=1), only the higher 16 bits (AWLT\\[23:8\\]) define the 16-bit threshold as compared with the analog watchdog filter output (because data coming from the analog watchdog filter are up to a 16-bit resolution). Bits AWLT\\[7:0\\]
are not taken into comparison in this case."]
pub type AWLT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event BKAWL\\[i\\]
= 0: Break i signal is not assigned to an analog watchdog low threshold event BKAWL\\[i\\]
= 1: Break i signal is assigned to an analog watchdog low threshold event"]
    #[inline(always)]
    pub fn bkawl(&self) -> BKAWL_R {
        BKAWL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - Analog watchdog low threshold These bits are written by software to define the low threshold for the analog watchdog. Note: In case channel transceivers monitor (AWFSEL=1), only the higher 16 bits (AWLT\\[23:8\\]) define the 16-bit threshold as compared with the analog watchdog filter output (because data coming from the analog watchdog filter are up to a 16-bit resolution). Bits AWLT\\[7:0\\]
are not taken into comparison in this case."]
    #[inline(always)]
    pub fn awlt(&self) -> AWLT_R {
        AWLT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event BKAWL\\[i\\]
= 0: Break i signal is not assigned to an analog watchdog low threshold event BKAWL\\[i\\]
= 1: Break i signal is assigned to an analog watchdog low threshold event"]
    #[inline(always)]
    #[must_use]
    pub fn bkawl(&mut self) -> BKAWL_W<AWLTRrs> {
        BKAWL_W::new(self, 0)
    }
    #[doc = "Bits 8:31 - Analog watchdog low threshold These bits are written by software to define the low threshold for the analog watchdog. Note: In case channel transceivers monitor (AWFSEL=1), only the higher 16 bits (AWLT\\[23:8\\]) define the 16-bit threshold as compared with the analog watchdog filter output (because data coming from the analog watchdog filter are up to a 16-bit resolution). Bits AWLT\\[7:0\\]
are not taken into comparison in this case."]
    #[inline(always)]
    #[must_use]
    pub fn awlt(&mut self) -> AWLT_W<AWLTRrs> {
        AWLT_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awltr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awltr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWLTRrs;
impl crate::RegisterSpec for AWLTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awltr::R`](R) reader structure"]
impl crate::Readable for AWLTRrs {}
#[doc = "`write(|w| ..)` method takes [`awltr::W`](W) writer structure"]
impl crate::Writable for AWLTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWLTR to value 0"]
impl crate::Resettable for AWLTRrs {
    const RESET_VALUE: u32 = 0;
}
