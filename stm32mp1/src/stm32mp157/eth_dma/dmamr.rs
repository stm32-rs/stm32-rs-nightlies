///Register `DMAMR` reader
pub type R = crate::R<DMAMRrs>;
///Register `DMAMR` writer
pub type W = crate::W<DMAMRrs>;
///Field `SWR` reader - Software Reset
pub type SWR_R = crate::BitReader;
///Field `SWR` writer - Software Reset
pub type SWR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAA` reader - TAA
pub type TAA_R = crate::FieldReader;
///Field `TAA` writer - TAA
pub type TAA_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TXPR` reader - Transmit priority
pub type TXPR_R = crate::BitReader;
///Field `TXPR` writer - Transmit priority
pub type TXPR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PR` reader - Priority ratio
pub type PR_R = crate::FieldReader;
///Field `PR` writer - Priority ratio
pub type PR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `INTM` reader - Interrupt Mode
pub type INTM_R = crate::FieldReader;
///Field `INTM` writer - Interrupt Mode
pub type INTM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Software Reset
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:4 - TAA
    #[inline(always)]
    pub fn taa(&self) -> TAA_R {
        TAA_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bit 11 - Transmit priority
    #[inline(always)]
    pub fn txpr(&self) -> TXPR_R {
        TXPR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - Priority ratio
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:17 - Interrupt Mode
    #[inline(always)]
    pub fn intm(&self) -> INTM_R {
        INTM_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAMR")
            .field("swr", &self.swr())
            .field("taa", &self.taa())
            .field("txpr", &self.txpr())
            .field("pr", &self.pr())
            .field("intm", &self.intm())
            .finish()
    }
}
impl W {
    ///Bit 0 - Software Reset
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W<DMAMRrs> {
        SWR_W::new(self, 0)
    }
    ///Bits 2:4 - TAA
    #[inline(always)]
    pub fn taa(&mut self) -> TAA_W<DMAMRrs> {
        TAA_W::new(self, 2)
    }
    ///Bit 11 - Transmit priority
    #[inline(always)]
    pub fn txpr(&mut self) -> TXPR_W<DMAMRrs> {
        TXPR_W::new(self, 11)
    }
    ///Bits 12:14 - Priority ratio
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W<DMAMRrs> {
        PR_W::new(self, 12)
    }
    ///Bits 16:17 - Interrupt Mode
    #[inline(always)]
    pub fn intm(&mut self) -> INTM_W<DMAMRrs> {
        INTM_W::new(self, 16)
    }
}
/**DMA mode register

You can [`read`](crate::Reg::read) this register and get [`dmamr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_DMA:DMAMR)*/
pub struct DMAMRrs;
impl crate::RegisterSpec for DMAMRrs {
    type Ux = u32;
}
///`read()` method returns [`dmamr::R`](R) reader structure
impl crate::Readable for DMAMRrs {}
///`write(|w| ..)` method takes [`dmamr::W`](W) writer structure
impl crate::Writable for DMAMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMAMR to value 0x8000
impl crate::Resettable for DMAMRrs {
    const RESET_VALUE: u32 = 0x8000;
}