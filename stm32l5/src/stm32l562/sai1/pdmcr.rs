///Register `PDMCR` reader
pub type R = crate::R<PDMCRrs>;
///Register `PDMCR` writer
pub type W = crate::W<PDMCRrs>;
///Field `PDMEN` reader - PDM enable
pub type PDMEN_R = crate::BitReader;
///Field `PDMEN` writer - PDM enable
pub type PDMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MICNBR` reader - MICNBR
pub type MICNBR_R = crate::FieldReader;
///Field `MICNBR` writer - MICNBR
pub type MICNBR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CKEN(1-2)` reader - Clock enable of bitstream clock number %s
pub type CKEN_R = crate::BitReader;
///Field `CKEN(1-2)` writer - Clock enable of bitstream clock number %s
pub type CKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - PDM enable
    #[inline(always)]
    pub fn pdmen(&self) -> PDMEN_R {
        PDMEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:5 - MICNBR
    #[inline(always)]
    pub fn micnbr(&self) -> MICNBR_R {
        MICNBR_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Clock enable of bitstream clock number (1-2)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CKEN1` field.</div>
    #[inline(always)]
    pub fn cken(&self, n: u8) -> CKEN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CKEN_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Clock enable of bitstream clock number (1-2)
    #[inline(always)]
    pub fn cken_iter(&self) -> impl Iterator<Item = CKEN_R> + '_ {
        (0..2).map(move |n| CKEN_R::new(((self.bits >> (n + 8)) & 1) != 0))
    }
    ///Bit 8 - Clock enable of bitstream clock number 1
    #[inline(always)]
    pub fn cken1(&self) -> CKEN_R {
        CKEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Clock enable of bitstream clock number 2
    #[inline(always)]
    pub fn cken2(&self) -> CKEN_R {
        CKEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDMCR")
            .field("pdmen", &self.pdmen())
            .field("micnbr", &self.micnbr())
            .field("cken1", &self.cken1())
            .field("cken2", &self.cken2())
            .finish()
    }
}
impl W {
    ///Bit 0 - PDM enable
    #[inline(always)]
    pub fn pdmen(&mut self) -> PDMEN_W<'_, PDMCRrs> {
        PDMEN_W::new(self, 0)
    }
    ///Bits 4:5 - MICNBR
    #[inline(always)]
    pub fn micnbr(&mut self) -> MICNBR_W<'_, PDMCRrs> {
        MICNBR_W::new(self, 4)
    }
    ///Clock enable of bitstream clock number (1-2)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CKEN1` field.</div>
    #[inline(always)]
    pub fn cken(&mut self, n: u8) -> CKEN_W<'_, PDMCRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CKEN_W::new(self, n + 8)
    }
    ///Bit 8 - Clock enable of bitstream clock number 1
    #[inline(always)]
    pub fn cken1(&mut self) -> CKEN_W<'_, PDMCRrs> {
        CKEN_W::new(self, 8)
    }
    ///Bit 9 - Clock enable of bitstream clock number 2
    #[inline(always)]
    pub fn cken2(&mut self) -> CKEN_W<'_, PDMCRrs> {
        CKEN_W::new(self, 9)
    }
}
/**PDM control register

You can [`read`](crate::Reg::read) this register and get [`pdmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SAI1:PDMCR)*/
pub struct PDMCRrs;
impl crate::RegisterSpec for PDMCRrs {
    type Ux = u32;
}
///`read()` method returns [`pdmcr::R`](R) reader structure
impl crate::Readable for PDMCRrs {}
///`write(|w| ..)` method takes [`pdmcr::W`](W) writer structure
impl crate::Writable for PDMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDMCR to value 0
impl crate::Resettable for PDMCRrs {}
