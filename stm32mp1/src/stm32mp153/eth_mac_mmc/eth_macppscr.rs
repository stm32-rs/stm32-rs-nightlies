#[doc = "Register `ETH_MACPPSCR` reader"]
pub type R = crate::R<ETH_MACPPSCRrs>;
#[doc = "Register `ETH_MACPPSCR` writer"]
pub type W = crate::W<ETH_MACPPSCRrs>;
#[doc = "Field `PPSCTRL` reader - PPSCTRL"]
pub type PPSCTRL_R = crate::FieldReader;
#[doc = "Field `PPSCTRL` writer - PPSCTRL"]
pub type PPSCTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PPSEN0` reader - PPSEN0"]
pub type PPSEN0_R = crate::BitReader;
#[doc = "Field `PPSEN0` writer - PPSEN0"]
pub type PPSEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGTMODSEL0` reader - TRGTMODSEL0"]
pub type TRGTMODSEL0_R = crate::FieldReader;
#[doc = "Field `TRGTMODSEL0` writer - TRGTMODSEL0"]
pub type TRGTMODSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - PPSCTRL"]
    #[inline(always)]
    pub fn ppsctrl(&self) -> PPSCTRL_R {
        PPSCTRL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - PPSEN0"]
    #[inline(always)]
    pub fn ppsen0(&self) -> PPSEN0_R {
        PPSEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - TRGTMODSEL0"]
    #[inline(always)]
    pub fn trgtmodsel0(&self) -> TRGTMODSEL0_R {
        TRGTMODSEL0_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PPSCTRL"]
    #[inline(always)]
    #[must_use]
    pub fn ppsctrl(&mut self) -> PPSCTRL_W<ETH_MACPPSCRrs> {
        PPSCTRL_W::new(self, 0)
    }
    #[doc = "Bit 4 - PPSEN0"]
    #[inline(always)]
    #[must_use]
    pub fn ppsen0(&mut self) -> PPSEN0_W<ETH_MACPPSCRrs> {
        PPSEN0_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - TRGTMODSEL0"]
    #[inline(always)]
    #[must_use]
    pub fn trgtmodsel0(&mut self) -> TRGTMODSEL0_W<ETH_MACPPSCRrs> {
        TRGTMODSEL0_W::new(self, 5)
    }
}
#[doc = "The PPS Control register is present only when the Timestamp feature is selected and External Timestamp is not enabled. Bits\\[30:24\\]
of this register are valid only when four Flexible PPS outputs are selected. Bits\\[22:16\\]
are valid only when three or more Flexible PPS outputs are selected. Bits\\[14:8\\]
are valid only when two or more Flexible PPS outputs are selected. Bits\\[6:4\\]
are valid only when Flexible PPS feature is selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macppscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macppscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACPPSCRrs;
impl crate::RegisterSpec for ETH_MACPPSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macppscr::R`](R) reader structure"]
impl crate::Readable for ETH_MACPPSCRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macppscr::W`](W) writer structure"]
impl crate::Writable for ETH_MACPPSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACPPSCR to value 0"]
impl crate::Resettable for ETH_MACPPSCRrs {
    const RESET_VALUE: u32 = 0;
}
