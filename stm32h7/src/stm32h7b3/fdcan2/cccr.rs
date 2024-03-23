#[doc = "Register `CCCR` reader"]
pub type R = crate::R<CCCRrs>;
#[doc = "Register `CCCR` writer"]
pub type W = crate::W<CCCRrs>;
#[doc = "Field `INIT` reader - Initialization"]
pub type INIT_R = crate::BitReader;
#[doc = "Field `INIT` writer - Initialization"]
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCE` reader - Configuration Change Enable"]
pub type CCE_R = crate::BitReader;
#[doc = "Field `CCE` writer - Configuration Change Enable"]
pub type CCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASM` reader - ASM Restricted Operation Mode"]
pub type ASM_R = crate::BitReader;
#[doc = "Field `ASM` writer - ASM Restricted Operation Mode"]
pub type ASM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSA` reader - Clock Stop Acknowledge"]
pub type CSA_R = crate::BitReader;
#[doc = "Field `CSA` writer - Clock Stop Acknowledge"]
pub type CSA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSR` reader - Clock Stop Request"]
pub type CSR_R = crate::BitReader;
#[doc = "Field `CSR` writer - Clock Stop Request"]
pub type CSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON` reader - Bus Monitoring Mode"]
pub type MON_R = crate::BitReader;
#[doc = "Field `MON` writer - Bus Monitoring Mode"]
pub type MON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAR` reader - Disable Automatic Retransmission"]
pub type DAR_R = crate::BitReader;
#[doc = "Field `DAR` writer - Disable Automatic Retransmission"]
pub type DAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST` reader - Test Mode Enable"]
pub type TEST_R = crate::BitReader;
#[doc = "Field `TEST` writer - Test Mode Enable"]
pub type TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDOE` reader - FD Operation Enable"]
pub type FDOE_R = crate::BitReader;
#[doc = "Field `FDOE` writer - FD Operation Enable"]
pub type FDOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSE` reader - FDCAN Bit Rate Switching"]
pub type BSE_R = crate::BitReader;
#[doc = "Field `BSE` writer - FDCAN Bit Rate Switching"]
pub type BSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PXHD` reader - Protocol Exception Handling Disable"]
pub type PXHD_R = crate::BitReader;
#[doc = "Field `PXHD` writer - Protocol Exception Handling Disable"]
pub type PXHD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFBI` reader - Edge Filtering during Bus Integration"]
pub type EFBI_R = crate::BitReader;
#[doc = "Field `EFBI` writer - Edge Filtering during Bus Integration"]
pub type EFBI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXP` reader - TXP"]
pub type TXP_R = crate::BitReader;
#[doc = "Field `TXP` writer - TXP"]
pub type TXP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NISO` reader - Non ISO Operation"]
pub type NISO_R = crate::BitReader;
#[doc = "Field `NISO` writer - Non ISO Operation"]
pub type NISO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Initialization"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configuration Change Enable"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ASM Restricted Operation Mode"]
    #[inline(always)]
    pub fn asm(&self) -> ASM_R {
        ASM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge"]
    #[inline(always)]
    pub fn csa(&self) -> CSA_R {
        CSA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock Stop Request"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bus Monitoring Mode"]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test Mode Enable"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FD Operation Enable"]
    #[inline(always)]
    pub fn fdoe(&self) -> FDOE_R {
        FDOE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FDCAN Bit Rate Switching"]
    #[inline(always)]
    pub fn bse(&self) -> BSE_R {
        BSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Protocol Exception Handling Disable"]
    #[inline(always)]
    pub fn pxhd(&self) -> PXHD_R {
        PXHD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Edge Filtering during Bus Integration"]
    #[inline(always)]
    pub fn efbi(&self) -> EFBI_R {
        EFBI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TXP"]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Non ISO Operation"]
    #[inline(always)]
    pub fn niso(&self) -> NISO_R {
        NISO_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialization"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<CCCRrs> {
        INIT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configuration Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CCE_W<CCCRrs> {
        CCE_W::new(self, 1)
    }
    #[doc = "Bit 2 - ASM Restricted Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn asm(&mut self) -> ASM_W<CCCRrs> {
        ASM_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn csa(&mut self) -> CSA_W<CCCRrs> {
        CSA_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clock Stop Request"]
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CSR_W<CCCRrs> {
        CSR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bus Monitoring Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mon(&mut self) -> MON_W<CCCRrs> {
        MON_W::new(self, 5)
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DAR_W<CCCRrs> {
        DAR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Test Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn test(&mut self) -> TEST_W<CCCRrs> {
        TEST_W::new(self, 7)
    }
    #[doc = "Bit 8 - FD Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdoe(&mut self) -> FDOE_W<CCCRrs> {
        FDOE_W::new(self, 8)
    }
    #[doc = "Bit 9 - FDCAN Bit Rate Switching"]
    #[inline(always)]
    #[must_use]
    pub fn bse(&mut self) -> BSE_W<CCCRrs> {
        BSE_W::new(self, 9)
    }
    #[doc = "Bit 12 - Protocol Exception Handling Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pxhd(&mut self) -> PXHD_W<CCCRrs> {
        PXHD_W::new(self, 12)
    }
    #[doc = "Bit 13 - Edge Filtering during Bus Integration"]
    #[inline(always)]
    #[must_use]
    pub fn efbi(&mut self) -> EFBI_W<CCCRrs> {
        EFBI_W::new(self, 13)
    }
    #[doc = "Bit 14 - TXP"]
    #[inline(always)]
    #[must_use]
    pub fn txp(&mut self) -> TXP_W<CCCRrs> {
        TXP_W::new(self, 14)
    }
    #[doc = "Bit 15 - Non ISO Operation"]
    #[inline(always)]
    #[must_use]
    pub fn niso(&mut self) -> NISO_W<CCCRrs> {
        NISO_W::new(self, 15)
    }
}
#[doc = "FDCAN CC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCCRrs;
impl crate::RegisterSpec for CCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cccr::R`](R) reader structure"]
impl crate::Readable for CCCRrs {}
#[doc = "`write(|w| ..)` method takes [`cccr::W`](W) writer structure"]
impl crate::Writable for CCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCCR to value 0x01"]
impl crate::Resettable for CCCRrs {
    const RESET_VALUE: u32 = 0x01;
}
