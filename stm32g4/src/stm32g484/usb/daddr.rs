///Register `DADDR` reader
pub type R = crate::R<DADDRrs>;
///Register `DADDR` writer
pub type W = crate::W<DADDRrs>;
///Field `ADD` reader - ADD
pub type ADD_R = crate::FieldReader;
///Field `ADD` writer - ADD
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `EF` reader - EF
pub type EF_R = crate::BitReader;
///Field `EF` writer - EF
pub type EF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:6 - ADD
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 7 - EF
    #[inline(always)]
    pub fn ef(&self) -> EF_R {
        EF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DADDR")
            .field("add", &self.add())
            .field("ef", &self.ef())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - ADD
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<DADDRrs> {
        ADD_W::new(self, 0)
    }
    ///Bit 7 - EF
    #[inline(always)]
    pub fn ef(&mut self) -> EF_W<DADDRrs> {
        EF_W::new(self, 7)
    }
}
/**USB device address

You can [`read`](crate::Reg::read) this register and get [`daddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#USB:DADDR)*/
pub struct DADDRrs;
impl crate::RegisterSpec for DADDRrs {
    type Ux = u32;
}
///`read()` method returns [`daddr::R`](R) reader structure
impl crate::Readable for DADDRrs {}
///`write(|w| ..)` method takes [`daddr::W`](W) writer structure
impl crate::Writable for DADDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DADDR to value 0
impl crate::Resettable for DADDRrs {}
