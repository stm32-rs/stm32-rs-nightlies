///Register `OTG_DIEPTXF1` reader
pub type R = crate::R<OTG_DIEPTXF1rs>;
///Register `OTG_DIEPTXF1` writer
pub type W = crate::W<OTG_DIEPTXF1rs>;
///Field `INEPTXSA` reader - INEPTXSA
pub type INEPTXSA_R = crate::FieldReader<u16>;
///Field `INEPTXSA` writer - INEPTXSA
pub type INEPTXSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `INEPTXFD` reader - INEPTXFD
pub type INEPTXFD_R = crate::FieldReader<u16>;
///Field `INEPTXFD` writer - INEPTXFD
pub type INEPTXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - INEPTXSA
    #[inline(always)]
    pub fn ineptxsa(&self) -> INEPTXSA_R {
        INEPTXSA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - INEPTXFD
    #[inline(always)]
    pub fn ineptxfd(&self) -> INEPTXFD_R {
        INEPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_DIEPTXF1")
            .field("ineptxsa", &self.ineptxsa())
            .field("ineptxfd", &self.ineptxfd())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - INEPTXSA
    #[inline(always)]
    #[must_use]
    pub fn ineptxsa(&mut self) -> INEPTXSA_W<OTG_DIEPTXF1rs> {
        INEPTXSA_W::new(self, 0)
    }
    ///Bits 16:31 - INEPTXFD
    #[inline(always)]
    #[must_use]
    pub fn ineptxfd(&mut self) -> INEPTXFD_W<OTG_DIEPTXF1rs> {
        INEPTXFD_W::new(self, 16)
    }
}
/**OTG device IN endpoint transmit FIFO 1 size register

You can [`read`](crate::Reg::read) this register and get [`otg_dieptxf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_dieptxf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#OTG:OTG_DIEPTXF1)*/
pub struct OTG_DIEPTXF1rs;
impl crate::RegisterSpec for OTG_DIEPTXF1rs {
    type Ux = u32;
}
///`read()` method returns [`otg_dieptxf1::R`](R) reader structure
impl crate::Readable for OTG_DIEPTXF1rs {}
///`write(|w| ..)` method takes [`otg_dieptxf1::W`](W) writer structure
impl crate::Writable for OTG_DIEPTXF1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OTG_DIEPTXF1 to value 0x0200_0400
impl crate::Resettable for OTG_DIEPTXF1rs {
    const RESET_VALUE: u32 = 0x0200_0400;
}
