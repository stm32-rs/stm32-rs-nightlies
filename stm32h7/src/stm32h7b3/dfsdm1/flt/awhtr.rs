#[doc = "Register `AWHTR` reader"]
pub type R = crate::R<AWHTRrs>;
#[doc = "Register `AWHTR` writer"]
pub type W = crate::W<AWHTRrs>;
#[doc = "Field `BKAWH` reader - Break signal assignment to analog watchdog high threshold event BKAWH\\[i\\]
= 0: Break i signal is not assigned to an analog watchdog high threshold event BKAWH\\[i\\]
= 1: Break i signal is assigned to an analog watchdog high threshold event"]
pub type BKAWH_R = crate::FieldReader;
#[doc = "Field `BKAWH` writer - Break signal assignment to analog watchdog high threshold event BKAWH\\[i\\]
= 0: Break i signal is not assigned to an analog watchdog high threshold event BKAWH\\[i\\]
= 1: Break i signal is assigned to an analog watchdog high threshold event"]
pub type BKAWH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AWHT` reader - Analog watchdog high threshold These bits are written by software to define the high threshold for the analog watchdog. Note: In case channel transceivers monitor (AWFSEL=1), the higher 16 bits (AWHT\\[23:8\\]) define the 16-bit threshold as compared with the analog watchdog filter output (because data coming from the analog watchdog filter are up to a 16-bit resolution). Bits AWHT\\[7:0\\]
are not taken into comparison in this case."]
pub type AWHT_R = crate::FieldReader<u32>;
#[doc = "Field `AWHT` writer - Analog watchdog high threshold These bits are written by software to define the high threshold for the analog watchdog. Note: In case channel transceivers monitor (AWFSEL=1), the higher 16 bits (AWHT\\[23:8\\]) define the 16-bit threshold as compared with the analog watchdog filter output (because data coming from the analog watchdog filter are up to a 16-bit resolution). Bits AWHT\\[7:0\\]
are not taken into comparison in this case."]
pub type AWHT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog high threshold event BKAWH\\[i\\]
= 0: Break i signal is not assigned to an analog watchdog high threshold event BKAWH\\[i\\]
= 1: Break i signal is assigned to an analog watchdog high threshold event"]
    #[inline(always)]
    pub fn bkawh(&self) -> BKAWH_R {
        BKAWH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - Analog watchdog high threshold These bits are written by software to define the high threshold for the analog watchdog. Note: In case channel transceivers monitor (AWFSEL=1), the higher 16 bits (AWHT\\[23:8\\]) define the 16-bit threshold as compared with the analog watchdog filter output (because data coming from the analog watchdog filter are up to a 16-bit resolution). Bits AWHT\\[7:0\\]
are not taken into comparison in this case."]
    #[inline(always)]
    pub fn awht(&self) -> AWHT_R {
        AWHT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog high threshold event BKAWH\\[i\\]
= 0: Break i signal is not assigned to an analog watchdog high threshold event BKAWH\\[i\\]
= 1: Break i signal is assigned to an analog watchdog high threshold event"]
    #[inline(always)]
    #[must_use]
    pub fn bkawh(&mut self) -> BKAWH_W<AWHTRrs> {
        BKAWH_W::new(self, 0)
    }
    #[doc = "Bits 8:31 - Analog watchdog high threshold These bits are written by software to define the high threshold for the analog watchdog. Note: In case channel transceivers monitor (AWFSEL=1), the higher 16 bits (AWHT\\[23:8\\]) define the 16-bit threshold as compared with the analog watchdog filter output (because data coming from the analog watchdog filter are up to a 16-bit resolution). Bits AWHT\\[7:0\\]
are not taken into comparison in this case."]
    #[inline(always)]
    #[must_use]
    pub fn awht(&mut self) -> AWHT_W<AWHTRrs> {
        AWHT_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awhtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awhtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWHTRrs;
impl crate::RegisterSpec for AWHTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awhtr::R`](R) reader structure"]
impl crate::Readable for AWHTRrs {}
#[doc = "`write(|w| ..)` method takes [`awhtr::W`](W) writer structure"]
impl crate::Writable for AWHTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWHTR to value 0"]
impl crate::Resettable for AWHTRrs {
    const RESET_VALUE: u32 = 0;
}
