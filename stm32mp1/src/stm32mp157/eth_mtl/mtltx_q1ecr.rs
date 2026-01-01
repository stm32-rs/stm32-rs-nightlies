///Register `MTLTxQ1ECR` reader
pub type R = crate::R<MTLTX_Q1ECRrs>;
///Register `MTLTxQ1ECR` writer
pub type W = crate::W<MTLTX_Q1ECRrs>;
///Field `AVALG` reader - AVALG
pub type AVALG_R = crate::BitReader;
///Field `AVALG` writer - AVALG
pub type AVALG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC` reader - CC
pub type CC_R = crate::BitReader;
///Field `CC` writer - CC
pub type CC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLC` reader - SLC
pub type SLC_R = crate::FieldReader;
///Field `SLC` writer - SLC
pub type SLC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 2 - AVALG
    #[inline(always)]
    pub fn avalg(&self) -> AVALG_R {
        AVALG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CC
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - SLC
    #[inline(always)]
    pub fn slc(&self) -> SLC_R {
        SLC_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTxQ1ECR")
            .field("avalg", &self.avalg())
            .field("cc", &self.cc())
            .field("slc", &self.slc())
            .finish()
    }
}
impl W {
    ///Bit 2 - AVALG
    #[inline(always)]
    pub fn avalg(&mut self) -> AVALG_W<'_, MTLTX_Q1ECRrs> {
        AVALG_W::new(self, 2)
    }
    ///Bit 3 - CC
    #[inline(always)]
    pub fn cc(&mut self) -> CC_W<'_, MTLTX_Q1ECRrs> {
        CC_W::new(self, 3)
    }
    ///Bits 4:6 - SLC
    #[inline(always)]
    pub fn slc(&mut self) -> SLC_W<'_, MTLTX_Q1ECRrs> {
        SLC_W::new(self, 4)
    }
}
/**The Queue ETS Control register controls the enhanced transmission selection operation.

You can [`read`](crate::Reg::read) this register and get [`mtltx_q1ecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltx_q1ecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:MTLTxQ1ECR)*/
pub struct MTLTX_Q1ECRrs;
impl crate::RegisterSpec for MTLTX_Q1ECRrs {
    type Ux = u32;
}
///`read()` method returns [`mtltx_q1ecr::R`](R) reader structure
impl crate::Readable for MTLTX_Q1ECRrs {}
///`write(|w| ..)` method takes [`mtltx_q1ecr::W`](W) writer structure
impl crate::Writable for MTLTX_Q1ECRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLTxQ1ECR to value 0
impl crate::Resettable for MTLTX_Q1ECRrs {}
