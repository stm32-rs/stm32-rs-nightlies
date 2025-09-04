///Register `C7LLR` reader
pub type R = crate::R<C7LLRrs>;
///Register `C7LLR` writer
pub type W = crate::W<C7LLRrs>;
///Field `LA` reader - pointer (16-bit low-significant address) to the next linked-list data structure
pub type LA_R = crate::FieldReader<u16>;
///Field `LA` writer - pointer (16-bit low-significant address) to the next linked-list data structure
pub type LA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `ULL` reader - Update HPDMA_CxLLR register from memory
pub type ULL_R = crate::BitReader;
///Field `ULL` writer - Update HPDMA_CxLLR register from memory
pub type ULL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDA` reader - Update HPDMA_CxDAR register from memory
pub type UDA_R = crate::BitReader;
///Field `UDA` writer - Update HPDMA_CxDAR register from memory
pub type UDA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USA` reader - update HPDMA_CxSAR from memory
pub type USA_R = crate::BitReader;
///Field `USA` writer - update HPDMA_CxSAR from memory
pub type USA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UB1` reader - Update HPDMA_CxBR1 from memory
pub type UB1_R = crate::BitReader;
///Field `UB1` writer - Update HPDMA_CxBR1 from memory
pub type UB1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UT2` reader - Update HPDMA_CxTR2 from memory
pub type UT2_R = crate::BitReader;
///Field `UT2` writer - Update HPDMA_CxTR2 from memory
pub type UT2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UT1` reader - Update HPDMA_CxTR1 from memory
pub type UT1_R = crate::BitReader;
///Field `UT1` writer - Update HPDMA_CxTR1 from memory
pub type UT1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 2:15 - pointer (16-bit low-significant address) to the next linked-list data structure
    #[inline(always)]
    pub fn la(&self) -> LA_R {
        LA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    ///Bit 16 - Update HPDMA_CxLLR register from memory
    #[inline(always)]
    pub fn ull(&self) -> ULL_R {
        ULL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 27 - Update HPDMA_CxDAR register from memory
    #[inline(always)]
    pub fn uda(&self) -> UDA_R {
        UDA_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - update HPDMA_CxSAR from memory
    #[inline(always)]
    pub fn usa(&self) -> USA_R {
        USA_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Update HPDMA_CxBR1 from memory
    #[inline(always)]
    pub fn ub1(&self) -> UB1_R {
        UB1_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Update HPDMA_CxTR2 from memory
    #[inline(always)]
    pub fn ut2(&self) -> UT2_R {
        UT2_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Update HPDMA_CxTR1 from memory
    #[inline(always)]
    pub fn ut1(&self) -> UT1_R {
        UT1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C7LLR")
            .field("la", &self.la())
            .field("ull", &self.ull())
            .field("uda", &self.uda())
            .field("usa", &self.usa())
            .field("ub1", &self.ub1())
            .field("ut2", &self.ut2())
            .field("ut1", &self.ut1())
            .finish()
    }
}
impl W {
    ///Bits 2:15 - pointer (16-bit low-significant address) to the next linked-list data structure
    #[inline(always)]
    pub fn la(&mut self) -> LA_W<C7LLRrs> {
        LA_W::new(self, 2)
    }
    ///Bit 16 - Update HPDMA_CxLLR register from memory
    #[inline(always)]
    pub fn ull(&mut self) -> ULL_W<C7LLRrs> {
        ULL_W::new(self, 16)
    }
    ///Bit 27 - Update HPDMA_CxDAR register from memory
    #[inline(always)]
    pub fn uda(&mut self) -> UDA_W<C7LLRrs> {
        UDA_W::new(self, 27)
    }
    ///Bit 28 - update HPDMA_CxSAR from memory
    #[inline(always)]
    pub fn usa(&mut self) -> USA_W<C7LLRrs> {
        USA_W::new(self, 28)
    }
    ///Bit 29 - Update HPDMA_CxBR1 from memory
    #[inline(always)]
    pub fn ub1(&mut self) -> UB1_W<C7LLRrs> {
        UB1_W::new(self, 29)
    }
    ///Bit 30 - Update HPDMA_CxTR2 from memory
    #[inline(always)]
    pub fn ut2(&mut self) -> UT2_W<C7LLRrs> {
        UT2_W::new(self, 30)
    }
    ///Bit 31 - Update HPDMA_CxTR1 from memory
    #[inline(always)]
    pub fn ut1(&mut self) -> UT1_W<C7LLRrs> {
        UT1_W::new(self, 31)
    }
}
/**HPDMA channel 7 linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c7llr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7llr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#HPDMA:C7LLR)*/
pub struct C7LLRrs;
impl crate::RegisterSpec for C7LLRrs {
    type Ux = u32;
}
///`read()` method returns [`c7llr::R`](R) reader structure
impl crate::Readable for C7LLRrs {}
///`write(|w| ..)` method takes [`c7llr::W`](W) writer structure
impl crate::Writable for C7LLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C7LLR to value 0
impl crate::Resettable for C7LLRrs {}
