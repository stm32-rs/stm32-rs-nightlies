#[doc = "Register `MACPPSTTNR` reader"]
pub type R = crate::R<MACPPSTTNRrs>;
#[doc = "Register `MACPPSTTNR` writer"]
pub type W = crate::W<MACPPSTTNRrs>;
#[doc = "Field `TTSL0` reader - Target Time Low for PPS Register"]
pub type TTSL0_R = crate::FieldReader<u32>;
#[doc = "Field `TTSL0` writer - Target Time Low for PPS Register"]
pub type TTSL0_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `TRGTBUSY0` reader - PPS Target Time Register Busy"]
pub type TRGTBUSY0_R = crate::BitReader;
#[doc = "Field `TRGTBUSY0` writer - PPS Target Time Register Busy"]
pub type TRGTBUSY0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - Target Time Low for PPS Register"]
    #[inline(always)]
    pub fn ttsl0(&self) -> TTSL0_R {
        TTSL0_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - PPS Target Time Register Busy"]
    #[inline(always)]
    pub fn trgtbusy0(&self) -> TRGTBUSY0_R {
        TRGTBUSY0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Target Time Low for PPS Register"]
    #[inline(always)]
    #[must_use]
    pub fn ttsl0(&mut self) -> TTSL0_W<MACPPSTTNRrs> {
        TTSL0_W::new(self, 0)
    }
    #[doc = "Bit 31 - PPS Target Time Register Busy"]
    #[inline(always)]
    #[must_use]
    pub fn trgtbusy0(&mut self) -> TRGTBUSY0_W<MACPPSTTNRrs> {
        TRGTBUSY0_W::new(self, 31)
    }
}
#[doc = "PPS target time nanoseconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macppsttnr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macppsttnr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACPPSTTNRrs;
impl crate::RegisterSpec for MACPPSTTNRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macppsttnr::R`](R) reader structure"]
impl crate::Readable for MACPPSTTNRrs {}
#[doc = "`write(|w| ..)` method takes [`macppsttnr::W`](W) writer structure"]
impl crate::Writable for MACPPSTTNRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACPPSTTNR to value 0"]
impl crate::Resettable for MACPPSTTNRrs {
    const RESET_VALUE: u32 = 0;
}
