///Register `DDR3_MR3` reader
pub type R = crate::R<DDR3_MR3rs>;
///Register `DDR3_MR3` writer
pub type W = crate::W<DDR3_MR3rs>;
///Field `MPRLOC` reader - MPRLOC
pub type MPRLOC_R = crate::FieldReader;
///Field `MPRLOC` writer - MPRLOC
pub type MPRLOC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MPR` reader - MPR
pub type MPR_R = crate::BitReader;
///Field `MPR` writer - MPR
pub type MPR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - MPRLOC
    #[inline(always)]
    pub fn mprloc(&self) -> MPRLOC_R {
        MPRLOC_R::new(self.bits & 3)
    }
    ///Bit 2 - MPR
    #[inline(always)]
    pub fn mpr(&self) -> MPR_R {
        MPR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDR3_MR3")
            .field("mprloc", &self.mprloc())
            .field("mpr", &self.mpr())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - MPRLOC
    #[inline(always)]
    pub fn mprloc(&mut self) -> MPRLOC_W<'_, DDR3_MR3rs> {
        MPRLOC_W::new(self, 0)
    }
    ///Bit 2 - MPR
    #[inline(always)]
    pub fn mpr(&mut self) -> MPR_W<'_, DDR3_MR3rs> {
        MPR_W::new(self, 2)
    }
}
/**DDRPHYC MR3 register for DDR3

You can [`read`](crate::Reg::read) this register and get [`ddr3_mr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddr3_mr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRPHYC:DDR3_MR3)*/
pub struct DDR3_MR3rs;
impl crate::RegisterSpec for DDR3_MR3rs {
    type Ux = u8;
}
///`read()` method returns [`ddr3_mr3::R`](R) reader structure
impl crate::Readable for DDR3_MR3rs {}
///`write(|w| ..)` method takes [`ddr3_mr3::W`](W) writer structure
impl crate::Writable for DDR3_MR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DDR3_MR3 to value 0
impl crate::Resettable for DDR3_MR3rs {}
