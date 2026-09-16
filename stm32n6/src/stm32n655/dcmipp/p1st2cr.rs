///Register `P1ST2CR` reader
pub type R = crate::R<P1ST2CRrs>;
///Register `P1ST2CR` writer
pub type W = crate::W<P1ST2CRrs>;
///Field `ENABLE` reader - None
pub type ENABLE_R = crate::BitReader;
///Field `ENABLE` writer - None
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BINS` reader - Bin definition
pub type BINS_R = crate::FieldReader;
///Field `BINS` writer - Bin definition
pub type BINS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SRC` reader - Statistics source
pub type SRC_R = crate::FieldReader;
///Field `SRC` writer - Statistics source
pub type SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MODE` reader - Statistics mode
pub type MODE_R = crate::BitReader;
///Field `MODE` writer - Statistics mode
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - None
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:3 - Bin definition
    #[inline(always)]
    pub fn bins(&self) -> BINS_R {
        BINS_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:6 - Statistics source
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Statistics mode
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1ST2CR")
            .field("enable", &self.enable())
            .field("bins", &self.bins())
            .field("src", &self.src())
            .field("mode", &self.mode())
            .finish()
    }
}
impl W {
    ///Bit 0 - None
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<'_, P1ST2CRrs> {
        ENABLE_W::new(self, 0)
    }
    ///Bits 2:3 - Bin definition
    #[inline(always)]
    pub fn bins(&mut self) -> BINS_W<'_, P1ST2CRrs> {
        BINS_W::new(self, 2)
    }
    ///Bits 4:6 - Statistics source
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W<'_, P1ST2CRrs> {
        SRC_W::new(self, 4)
    }
    ///Bit 7 - Statistics mode
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<'_, P1ST2CRrs> {
        MODE_W::new(self, 7)
    }
}
/**DCMIPP Pipe1 statistics 2 control register

You can [`read`](crate::Reg::read) this register and get [`p1st2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1st2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P1ST2CR)*/
pub struct P1ST2CRrs;
impl crate::RegisterSpec for P1ST2CRrs {
    type Ux = u32;
}
///`read()` method returns [`p1st2cr::R`](R) reader structure
impl crate::Readable for P1ST2CRrs {}
///`write(|w| ..)` method takes [`p1st2cr::W`](W) writer structure
impl crate::Writable for P1ST2CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1ST2CR to value 0
impl crate::Resettable for P1ST2CRrs {}
