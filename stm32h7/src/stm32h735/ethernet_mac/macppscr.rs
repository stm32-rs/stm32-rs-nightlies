#[doc = "Register `MACPPSCR` reader"]
pub type R = crate::R<MACPPSCRrs>;
#[doc = "Register `MACPPSCR` writer"]
pub type W = crate::W<MACPPSCRrs>;
#[doc = "Field `PPSCTRL` reader - Flexible PPS Output (ptp_pps_o\\[0\\]) Control or PPSCTRL PPS Output Frequency Control if PPSEN0 is cleared"]
pub type PPSCTRL_R = crate::FieldReader;
#[doc = "Field `PPSCTRL` writer - Flexible PPS Output (ptp_pps_o\\[0\\]) Control or PPSCTRL PPS Output Frequency Control if PPSEN0 is cleared"]
pub type PPSCTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PPSEN0` reader - Flexible PPS Output Mode Enable"]
pub type PPSEN0_R = crate::BitReader;
#[doc = "Field `PPSEN0` writer - Flexible PPS Output Mode Enable"]
pub type PPSEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGTMODSEL0` reader - Target Time Register Mode for PPS Output"]
pub type TRGTMODSEL0_R = crate::FieldReader;
#[doc = "Field `TRGTMODSEL0` writer - Target Time Register Mode for PPS Output"]
pub type TRGTMODSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Flexible PPS Output (ptp_pps_o\\[0\\]) Control or PPSCTRL PPS Output Frequency Control if PPSEN0 is cleared"]
    #[inline(always)]
    pub fn ppsctrl(&self) -> PPSCTRL_R {
        PPSCTRL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable"]
    #[inline(always)]
    pub fn ppsen0(&self) -> PPSEN0_R {
        PPSEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Target Time Register Mode for PPS Output"]
    #[inline(always)]
    pub fn trgtmodsel0(&self) -> TRGTMODSEL0_R {
        TRGTMODSEL0_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Flexible PPS Output (ptp_pps_o\\[0\\]) Control or PPSCTRL PPS Output Frequency Control if PPSEN0 is cleared"]
    #[inline(always)]
    #[must_use]
    pub fn ppsctrl(&mut self) -> PPSCTRL_W<MACPPSCRrs> {
        PPSCTRL_W::new(self, 0)
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ppsen0(&mut self) -> PPSEN0_W<MACPPSCRrs> {
        PPSEN0_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Target Time Register Mode for PPS Output"]
    #[inline(always)]
    #[must_use]
    pub fn trgtmodsel0(&mut self) -> TRGTMODSEL0_W<MACPPSCRrs> {
        TRGTMODSEL0_W::new(self, 5)
    }
}
#[doc = "PPS control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macppscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macppscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACPPSCRrs;
impl crate::RegisterSpec for MACPPSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macppscr::R`](R) reader structure"]
impl crate::Readable for MACPPSCRrs {}
#[doc = "`write(|w| ..)` method takes [`macppscr::W`](W) writer structure"]
impl crate::Writable for MACPPSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACPPSCR to value 0"]
impl crate::Resettable for MACPPSCRrs {
    const RESET_VALUE: u32 = 0;
}
