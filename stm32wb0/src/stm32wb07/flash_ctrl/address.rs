///Register `ADDRESS` reader
pub type R = crate::R<ADDRESSrs>;
///Register `ADDRESS` writer
pub type W = crate::W<ADDRESSrs>;
///Field `YADDR` reader - Flash column address offset to be used with some COMMAND
pub type YADDR_R = crate::FieldReader;
///Field `YADDR` writer - Flash column address offset to be used with some COMMAND
pub type YADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `XADDR` reader - Flash row address offset to be used with some COMMAND
pub type XADDR_R = crate::FieldReader<u16>;
///Field `XADDR` writer - Flash row address offset to be used with some COMMAND
pub type XADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:5 - Flash column address offset to be used with some COMMAND
    #[inline(always)]
    pub fn yaddr(&self) -> YADDR_R {
        YADDR_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:15 - Flash row address offset to be used with some COMMAND
    #[inline(always)]
    pub fn xaddr(&self) -> XADDR_R {
        XADDR_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDRESS")
            .field("yaddr", &self.yaddr())
            .field("xaddr", &self.xaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Flash column address offset to be used with some COMMAND
    #[inline(always)]
    pub fn yaddr(&mut self) -> YADDR_W<'_, ADDRESSrs> {
        YADDR_W::new(self, 0)
    }
    ///Bits 6:15 - Flash row address offset to be used with some COMMAND
    #[inline(always)]
    pub fn xaddr(&mut self) -> XADDR_W<'_, ADDRESSrs> {
        XADDR_W::new(self, 6)
    }
}
/**ADDRESS register

You can [`read`](crate::Reg::read) this register and get [`address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#FLASH_CTRL:ADDRESS)*/
pub struct ADDRESSrs;
impl crate::RegisterSpec for ADDRESSrs {
    type Ux = u32;
}
///`read()` method returns [`address::R`](R) reader structure
impl crate::Readable for ADDRESSrs {}
///`write(|w| ..)` method takes [`address::W`](W) writer structure
impl crate::Writable for ADDRESSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDRESS to value 0
impl crate::Resettable for ADDRESSrs {}
