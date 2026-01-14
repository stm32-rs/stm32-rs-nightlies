///Register `C12LLR` reader
pub type R = crate::R<C12LLRrs>;
///Register `C12LLR` writer
pub type W = crate::W<C12LLRrs>;
///Field `LA` reader - pointer (16-bit low-significant address) to the next linked-list data structure
pub type LA_R = crate::FieldReader<u16>;
///Field `LA` writer - pointer (16-bit low-significant address) to the next linked-list data structure
pub type LA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `ULL` reader - Update HPDMA_CxLLR register from memory
pub type ULL_R = crate::BitReader;
///Field `ULL` writer - Update HPDMA_CxLLR register from memory
pub type ULL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UB2` reader - Update HPDMA_CxBR2 from memory
pub type UB2_R = crate::BitReader;
///Field `UB2` writer - Update HPDMA_CxBR2 from memory
pub type UB2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UT3` reader - Update HPDMA_CxTR3 from memory
pub type UT3_R = crate::BitReader;
///Field `UT3` writer - Update HPDMA_CxTR3 from memory
pub type UT3_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 25 - Update HPDMA_CxBR2 from memory
    #[inline(always)]
    pub fn ub2(&self) -> UB2_R {
        UB2_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Update HPDMA_CxTR3 from memory
    #[inline(always)]
    pub fn ut3(&self) -> UT3_R {
        UT3_R::new(((self.bits >> 26) & 1) != 0)
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
        f.debug_struct("C12LLR")
            .field("la", &self.la())
            .field("ull", &self.ull())
            .field("ub2", &self.ub2())
            .field("ut3", &self.ut3())
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
    pub fn la(&mut self) -> LA_W<'_, C12LLRrs> {
        LA_W::new(self, 2)
    }
    ///Bit 16 - Update HPDMA_CxLLR register from memory
    #[inline(always)]
    pub fn ull(&mut self) -> ULL_W<'_, C12LLRrs> {
        ULL_W::new(self, 16)
    }
    ///Bit 25 - Update HPDMA_CxBR2 from memory
    #[inline(always)]
    pub fn ub2(&mut self) -> UB2_W<'_, C12LLRrs> {
        UB2_W::new(self, 25)
    }
    ///Bit 26 - Update HPDMA_CxTR3 from memory
    #[inline(always)]
    pub fn ut3(&mut self) -> UT3_W<'_, C12LLRrs> {
        UT3_W::new(self, 26)
    }
    ///Bit 27 - Update HPDMA_CxDAR register from memory
    #[inline(always)]
    pub fn uda(&mut self) -> UDA_W<'_, C12LLRrs> {
        UDA_W::new(self, 27)
    }
    ///Bit 28 - update HPDMA_CxSAR from memory
    #[inline(always)]
    pub fn usa(&mut self) -> USA_W<'_, C12LLRrs> {
        USA_W::new(self, 28)
    }
    ///Bit 29 - Update HPDMA_CxBR1 from memory
    #[inline(always)]
    pub fn ub1(&mut self) -> UB1_W<'_, C12LLRrs> {
        UB1_W::new(self, 29)
    }
    ///Bit 30 - Update HPDMA_CxTR2 from memory
    #[inline(always)]
    pub fn ut2(&mut self) -> UT2_W<'_, C12LLRrs> {
        UT2_W::new(self, 30)
    }
    ///Bit 31 - Update HPDMA_CxTR1 from memory
    #[inline(always)]
    pub fn ut1(&mut self) -> UT1_W<'_, C12LLRrs> {
        UT1_W::new(self, 31)
    }
}
/**HPDMA channel 12 alternate linked-list address register

You can [`read`](crate::Reg::read) this register and get [`c12llr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12llr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#HPDMA:C12LLR)*/
pub struct C12LLRrs;
impl crate::RegisterSpec for C12LLRrs {
    type Ux = u32;
}
///`read()` method returns [`c12llr::R`](R) reader structure
impl crate::Readable for C12LLRrs {}
///`write(|w| ..)` method takes [`c12llr::W`](W) writer structure
impl crate::Writable for C12LLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C12LLR to value 0
impl crate::Resettable for C12LLRrs {}
