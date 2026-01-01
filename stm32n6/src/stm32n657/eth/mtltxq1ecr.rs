///Register `MTLTXQ1ECR` reader
pub type R = crate::R<MTLTXQ1ECRrs>;
///Register `MTLTXQ1ECR` writer
pub type W = crate::W<MTLTXQ1ECRrs>;
///Field `AVALG` reader - AV Algorithm
pub type AVALG_R = crate::BitReader;
///Field `AVALG` writer - AV Algorithm
pub type AVALG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC` reader - Credit Control
pub type CC_R = crate::BitReader;
///Field `CC` writer - Credit Control
pub type CC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLC` reader - Slot Count
pub type SLC_R = crate::FieldReader;
///Field `SLC` writer - Slot Count
pub type SLC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 2 - AV Algorithm
    #[inline(always)]
    pub fn avalg(&self) -> AVALG_R {
        AVALG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Credit Control
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Slot Count
    #[inline(always)]
    pub fn slc(&self) -> SLC_R {
        SLC_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTXQ1ECR")
            .field("avalg", &self.avalg())
            .field("cc", &self.cc())
            .field("slc", &self.slc())
            .finish()
    }
}
impl W {
    ///Bit 2 - AV Algorithm
    #[inline(always)]
    pub fn avalg(&mut self) -> AVALG_W<'_, MTLTXQ1ECRrs> {
        AVALG_W::new(self, 2)
    }
    ///Bit 3 - Credit Control
    #[inline(always)]
    pub fn cc(&mut self) -> CC_W<'_, MTLTXQ1ECRrs> {
        CC_W::new(self, 3)
    }
    ///Bits 4:6 - Slot Count
    #[inline(always)]
    pub fn slc(&mut self) -> SLC_W<'_, MTLTXQ1ECRrs> {
        SLC_W::new(self, 4)
    }
}
/**Tx queue 1 ETS control Register

You can [`read`](crate::Reg::read) this register and get [`mtltxq1ecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxq1ecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ETH:MTLTXQ1ECR)*/
pub struct MTLTXQ1ECRrs;
impl crate::RegisterSpec for MTLTXQ1ECRrs {
    type Ux = u32;
}
///`read()` method returns [`mtltxq1ecr::R`](R) reader structure
impl crate::Readable for MTLTXQ1ECRrs {}
///`write(|w| ..)` method takes [`mtltxq1ecr::W`](W) writer structure
impl crate::Writable for MTLTXQ1ECRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLTXQ1ECR to value 0
impl crate::Resettable for MTLTXQ1ECRrs {}
