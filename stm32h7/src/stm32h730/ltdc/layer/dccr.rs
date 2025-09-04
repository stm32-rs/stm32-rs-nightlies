///Register `DCCR` reader
pub type R = crate::R<DCCRrs>;
///Register `DCCR` writer
pub type W = crate::W<DCCRrs>;
///Field `DCBLUE` reader - Default Color Blue
pub type DCBLUE_R = crate::FieldReader;
///Field `DCBLUE` writer - Default Color Blue
pub type DCBLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `DCGREEN` reader - Default Color Green
pub type DCGREEN_R = crate::FieldReader;
///Field `DCGREEN` writer - Default Color Green
pub type DCGREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `DCRED` reader - Default Color Red
pub type DCRED_R = crate::FieldReader;
///Field `DCRED` writer - Default Color Red
pub type DCRED_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `DCALPHA` reader - Default Color Alpha
pub type DCALPHA_R = crate::FieldReader;
///Field `DCALPHA` writer - Default Color Alpha
pub type DCALPHA_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:7 - Default Color Blue
    #[inline(always)]
    pub fn dcblue(&self) -> DCBLUE_R {
        DCBLUE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Default Color Green
    #[inline(always)]
    pub fn dcgreen(&self) -> DCGREEN_R {
        DCGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Default Color Red
    #[inline(always)]
    pub fn dcred(&self) -> DCRED_R {
        DCRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Default Color Alpha
    #[inline(always)]
    pub fn dcalpha(&self) -> DCALPHA_R {
        DCALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCCR")
            .field("dcalpha", &self.dcalpha())
            .field("dcred", &self.dcred())
            .field("dcgreen", &self.dcgreen())
            .field("dcblue", &self.dcblue())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Default Color Blue
    #[inline(always)]
    pub fn dcblue(&mut self) -> DCBLUE_W<DCCRrs> {
        DCBLUE_W::new(self, 0)
    }
    ///Bits 8:15 - Default Color Green
    #[inline(always)]
    pub fn dcgreen(&mut self) -> DCGREEN_W<DCCRrs> {
        DCGREEN_W::new(self, 8)
    }
    ///Bits 16:23 - Default Color Red
    #[inline(always)]
    pub fn dcred(&mut self) -> DCRED_W<DCCRrs> {
        DCRED_W::new(self, 16)
    }
    ///Bits 24:31 - Default Color Alpha
    #[inline(always)]
    pub fn dcalpha(&mut self) -> DCALPHA_W<DCCRrs> {
        DCALPHA_W::new(self, 24)
    }
}
/**Layerx Default Color Configuration Register

You can [`read`](crate::Reg::read) this register and get [`dccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DCCRrs;
impl crate::RegisterSpec for DCCRrs {
    type Ux = u32;
}
///`read()` method returns [`dccr::R`](R) reader structure
impl crate::Readable for DCCRrs {}
///`write(|w| ..)` method takes [`dccr::W`](W) writer structure
impl crate::Writable for DCCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCCR to value 0
impl crate::Resettable for DCCRrs {}
