#[doc = "Register `PCSCNTR` reader"]
pub type R = crate::R<PCSCNTRrs>;
#[doc = "Register `PCSCNTR` writer"]
pub type W = crate::W<PCSCNTRrs>;
#[doc = "Field `CSCOUNT` reader - Chip select counter. These bits are written by software to define the maximum chip select low pulse duration. It is expressed in FMC_CLK cycles for synchronous accesses and in HCLK cycles for asynchronous accesses. The counter is disabled if the programmed value is 0."]
pub type CSCOUNT_R = crate::FieldReader<u16>;
#[doc = "Field `CSCOUNT` writer - Chip select counter. These bits are written by software to define the maximum chip select low pulse duration. It is expressed in FMC_CLK cycles for synchronous accesses and in HCLK cycles for asynchronous accesses. The counter is disabled if the programmed value is 0."]
pub type CSCOUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CNTB1EN` reader - Counter Bank 1 enable This bit enables the chip select counter for PSRAM/NOR Bank 1."]
pub type CNTB1EN_R = crate::BitReader;
#[doc = "Field `CNTB1EN` writer - Counter Bank 1 enable This bit enables the chip select counter for PSRAM/NOR Bank 1."]
pub type CNTB1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTB2EN` reader - Counter Bank 2 enable This bit enables the chip select counter for PSRAM/NOR Bank 2."]
pub type CNTB2EN_R = crate::BitReader;
#[doc = "Field `CNTB2EN` writer - Counter Bank 2 enable This bit enables the chip select counter for PSRAM/NOR Bank 2."]
pub type CNTB2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTB3EN` reader - Counter Bank 3 enable This bit enables the chip select counter for PSRAM/NOR Bank 3."]
pub type CNTB3EN_R = crate::BitReader;
#[doc = "Field `CNTB3EN` writer - Counter Bank 3 enable This bit enables the chip select counter for PSRAM/NOR Bank 3."]
pub type CNTB3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTB4EN` reader - Counter Bank 4 enable This bit enables the chip select counter for PSRAM/NOR Bank 4."]
pub type CNTB4EN_R = crate::BitReader;
#[doc = "Field `CNTB4EN` writer - Counter Bank 4 enable This bit enables the chip select counter for PSRAM/NOR Bank 4."]
pub type CNTB4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Chip select counter. These bits are written by software to define the maximum chip select low pulse duration. It is expressed in FMC_CLK cycles for synchronous accesses and in HCLK cycles for asynchronous accesses. The counter is disabled if the programmed value is 0."]
    #[inline(always)]
    pub fn cscount(&self) -> CSCOUNT_R {
        CSCOUNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Counter Bank 1 enable This bit enables the chip select counter for PSRAM/NOR Bank 1."]
    #[inline(always)]
    pub fn cntb1en(&self) -> CNTB1EN_R {
        CNTB1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Counter Bank 2 enable This bit enables the chip select counter for PSRAM/NOR Bank 2."]
    #[inline(always)]
    pub fn cntb2en(&self) -> CNTB2EN_R {
        CNTB2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Counter Bank 3 enable This bit enables the chip select counter for PSRAM/NOR Bank 3."]
    #[inline(always)]
    pub fn cntb3en(&self) -> CNTB3EN_R {
        CNTB3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Counter Bank 4 enable This bit enables the chip select counter for PSRAM/NOR Bank 4."]
    #[inline(always)]
    pub fn cntb4en(&self) -> CNTB4EN_R {
        CNTB4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Chip select counter. These bits are written by software to define the maximum chip select low pulse duration. It is expressed in FMC_CLK cycles for synchronous accesses and in HCLK cycles for asynchronous accesses. The counter is disabled if the programmed value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn cscount(&mut self) -> CSCOUNT_W<PCSCNTRrs> {
        CSCOUNT_W::new(self, 0)
    }
    #[doc = "Bit 16 - Counter Bank 1 enable This bit enables the chip select counter for PSRAM/NOR Bank 1."]
    #[inline(always)]
    #[must_use]
    pub fn cntb1en(&mut self) -> CNTB1EN_W<PCSCNTRrs> {
        CNTB1EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Counter Bank 2 enable This bit enables the chip select counter for PSRAM/NOR Bank 2."]
    #[inline(always)]
    #[must_use]
    pub fn cntb2en(&mut self) -> CNTB2EN_W<PCSCNTRrs> {
        CNTB2EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Counter Bank 3 enable This bit enables the chip select counter for PSRAM/NOR Bank 3."]
    #[inline(always)]
    #[must_use]
    pub fn cntb3en(&mut self) -> CNTB3EN_W<PCSCNTRrs> {
        CNTB3EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Counter Bank 4 enable This bit enables the chip select counter for PSRAM/NOR Bank 4."]
    #[inline(always)]
    #[must_use]
    pub fn cntb4en(&mut self) -> CNTB4EN_W<PCSCNTRrs> {
        CNTB4EN_W::new(self, 19)
    }
}
#[doc = "PSRAM chip select counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcscntr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcscntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCSCNTRrs;
impl crate::RegisterSpec for PCSCNTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcscntr::R`](R) reader structure"]
impl crate::Readable for PCSCNTRrs {}
#[doc = "`write(|w| ..)` method takes [`pcscntr::W`](W) writer structure"]
impl crate::Writable for PCSCNTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCSCNTR to value 0"]
impl crate::Resettable for PCSCNTRrs {
    const RESET_VALUE: u32 = 0;
}
