///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `TSIZE` reader - Number of data at current transfer
pub type TSIZE_R = crate::FieldReader<u16>;
///Field `TSIZE` writer - Number of data at current transfer
pub type TSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
///Field `TSER` reader - Number of data transfer extension to be reload into TSIZE just when a previous
pub type TSER_R = crate::FieldReader<u16>;
///Field `TSER` writer - Number of data transfer extension to be reload into TSIZE just when a previous
pub type TSER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Number of data at current transfer
    #[inline(always)]
    pub fn tsize(&self) -> TSIZE_R {
        TSIZE_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Number of data transfer extension to be reload into TSIZE just when a previous
    #[inline(always)]
    pub fn tser(&self) -> TSER_R {
        TSER_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("tser", &self.tser())
            .field("tsize", &self.tsize())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Number of data at current transfer
    #[inline(always)]
    pub fn tsize(&mut self) -> TSIZE_W<'_, CR2rs> {
        TSIZE_W::new(self, 0)
    }
    ///Bits 16:31 - Number of data transfer extension to be reload into TSIZE just when a previous
    #[inline(always)]
    pub fn tser(&mut self) -> TSER_W<'_, CR2rs> {
        TSER_W::new(self, 16)
    }
}
/**control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#SPI1:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
