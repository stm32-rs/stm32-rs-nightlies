#[doc = "Register `ETH_MACPPSTTNR` reader"]
pub type R = crate::R<ETH_MACPPSTTNRrs>;
#[doc = "Register `ETH_MACPPSTTNR` writer"]
pub type W = crate::W<ETH_MACPPSTTNRrs>;
#[doc = "Field `TTSL0` reader - TTSL0"]
pub type TTSL0_R = crate::FieldReader<u32>;
#[doc = "Field `TTSL0` writer - TTSL0"]
pub type TTSL0_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `TRGTBUSY0` reader - TRGTBUSY0"]
pub type TRGTBUSY0_R = crate::BitReader;
#[doc = "Field `TRGTBUSY0` writer - TRGTBUSY0"]
pub type TRGTBUSY0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - TTSL0"]
    #[inline(always)]
    pub fn ttsl0(&self) -> TTSL0_R {
        TTSL0_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - TRGTBUSY0"]
    #[inline(always)]
    pub fn trgtbusy0(&self) -> TRGTBUSY0_R {
        TRGTBUSY0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - TTSL0"]
    #[inline(always)]
    #[must_use]
    pub fn ttsl0(&mut self) -> TTSL0_W<ETH_MACPPSTTNRrs> {
        TTSL0_W::new(self, 0)
    }
    #[doc = "Bit 31 - TRGTBUSY0"]
    #[inline(always)]
    #[must_use]
    pub fn trgtbusy0(&mut self) -> TRGTBUSY0_W<ETH_MACPPSTTNRrs> {
        TRGTBUSY0_W::new(self, 31)
    }
}
#[doc = "The PPS Target Time Nanoseconds register is present only when more than one Flexible PPS output is selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macppsttnr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macppsttnr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACPPSTTNRrs;
impl crate::RegisterSpec for ETH_MACPPSTTNRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macppsttnr::R`](R) reader structure"]
impl crate::Readable for ETH_MACPPSTTNRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macppsttnr::W`](W) writer structure"]
impl crate::Writable for ETH_MACPPSTTNRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACPPSTTNR to value 0"]
impl crate::Resettable for ETH_MACPPSTTNRrs {
    const RESET_VALUE: u32 = 0;
}
