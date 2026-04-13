///Register `OTPCR` reader
pub type R = crate::R<OTPCRrs>;
///Register `OTPCR` writer
pub type W = crate::W<OTPCRrs>;
///Field `ADDR` reader - Fuse word address
pub type ADDR_R = crate::FieldReader<u16>;
///Field `ADDR` writer - Fuse word address
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `PROG` reader - Fuse word programming
pub type PROG_R = crate::BitReader;
///Field `PROG` writer - Fuse word programming
pub type PROG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PPLOCK` reader - Permanent programming lock
pub type PPLOCK_R = crate::BitReader;
///Field `PPLOCK` writer - Permanent programming lock
pub type PPLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LASTCID` reader - Last CID
pub type LASTCID_R = crate::FieldReader;
impl R {
    ///Bits 0:8 - Fuse word address
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 13 - Fuse word programming
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Permanent programming lock
    #[inline(always)]
    pub fn pplock(&self) -> PPLOCK_R {
        PPLOCK_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 19:21 - Last CID
    #[inline(always)]
    pub fn lastcid(&self) -> LASTCID_R {
        LASTCID_R::new(((self.bits >> 19) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTPCR")
            .field("addr", &self.addr())
            .field("prog", &self.prog())
            .field("pplock", &self.pplock())
            .field("lastcid", &self.lastcid())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Fuse word address
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W<'_, OTPCRrs> {
        ADDR_W::new(self, 0)
    }
    ///Bit 13 - Fuse word programming
    #[inline(always)]
    pub fn prog(&mut self) -> PROG_W<'_, OTPCRrs> {
        PROG_W::new(self, 13)
    }
    ///Bit 14 - Permanent programming lock
    #[inline(always)]
    pub fn pplock(&mut self) -> PPLOCK_W<'_, OTPCRrs> {
        PPLOCK_W::new(self, 14)
    }
}
/**BSEC OTP control register

You can [`read`](crate::Reg::read) this register and get [`otpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:OTPCR)*/
pub struct OTPCRrs;
impl crate::RegisterSpec for OTPCRrs {
    type Ux = u32;
}
///`read()` method returns [`otpcr::R`](R) reader structure
impl crate::Readable for OTPCRrs {}
///`write(|w| ..)` method takes [`otpcr::W`](W) writer structure
impl crate::Writable for OTPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTPCR to value 0
impl crate::Resettable for OTPCRrs {}
