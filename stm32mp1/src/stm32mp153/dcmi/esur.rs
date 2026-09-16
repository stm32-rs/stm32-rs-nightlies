///Register `ESUR` reader
pub type R = crate::R<ESURrs>;
///Register `ESUR` writer
pub type W = crate::W<ESURrs>;
///Field `FSU` reader - FSU
pub type FSU_R = crate::FieldReader;
///Field `FSU` writer - FSU
pub type FSU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `LSU` reader - LSU
pub type LSU_R = crate::FieldReader;
///Field `LSU` writer - LSU
pub type LSU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `LEU` reader - LEU
pub type LEU_R = crate::FieldReader;
///Field `LEU` writer - LEU
pub type LEU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `FEU` reader - FEU
pub type FEU_R = crate::FieldReader;
///Field `FEU` writer - FEU
pub type FEU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - FSU
    #[inline(always)]
    pub fn fsu(&self) -> FSU_R {
        FSU_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - LSU
    #[inline(always)]
    pub fn lsu(&self) -> LSU_R {
        LSU_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - LEU
    #[inline(always)]
    pub fn leu(&self) -> LEU_R {
        LEU_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - FEU
    #[inline(always)]
    pub fn feu(&self) -> FEU_R {
        FEU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESUR")
            .field("fsu", &self.fsu())
            .field("lsu", &self.lsu())
            .field("leu", &self.leu())
            .field("feu", &self.feu())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - FSU
    #[inline(always)]
    pub fn fsu(&mut self) -> FSU_W<'_, ESURrs> {
        FSU_W::new(self, 0)
    }
    ///Bits 8:15 - LSU
    #[inline(always)]
    pub fn lsu(&mut self) -> LSU_W<'_, ESURrs> {
        LSU_W::new(self, 8)
    }
    ///Bits 16:23 - LEU
    #[inline(always)]
    pub fn leu(&mut self) -> LEU_W<'_, ESURrs> {
        LEU_W::new(self, 16)
    }
    ///Bits 24:31 - FEU
    #[inline(always)]
    pub fn feu(&mut self) -> FEU_W<'_, ESURrs> {
        FEU_W::new(self, 24)
    }
}
/**DCMI embedded synchronization unmask register

You can [`read`](crate::Reg::read) this register and get [`esur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DCMI:ESUR)*/
pub struct ESURrs;
impl crate::RegisterSpec for ESURrs {
    type Ux = u32;
}
///`read()` method returns [`esur::R`](R) reader structure
impl crate::Readable for ESURrs {}
///`write(|w| ..)` method takes [`esur::W`](W) writer structure
impl crate::Writable for ESURrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ESUR to value 0
impl crate::Resettable for ESURrs {}
