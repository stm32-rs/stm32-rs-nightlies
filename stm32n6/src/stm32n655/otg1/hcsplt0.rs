///Register `HCSPLT0` reader
pub type R = crate::R<HCSPLT0rs>;
///Register `HCSPLT0` writer
pub type W = crate::W<HCSPLT0rs>;
///Field `PRTADDR` reader - Port address
pub type PRTADDR_R = crate::FieldReader;
///Field `PRTADDR` writer - Port address
pub type PRTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `HUBADDR` reader - Hub address
pub type HUBADDR_R = crate::FieldReader;
///Field `HUBADDR` writer - Hub address
pub type HUBADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `XACTPOS` reader - Transaction position
pub type XACTPOS_R = crate::FieldReader;
///Field `XACTPOS` writer - Transaction position
pub type XACTPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `COMPLSPLT` reader - Do complete split
pub type COMPLSPLT_R = crate::BitReader;
///Field `COMPLSPLT` writer - Do complete split
pub type COMPLSPLT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPLITEN` reader - Split enable
pub type SPLITEN_R = crate::BitReader;
///Field `SPLITEN` writer - Split enable
pub type SPLITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:6 - Port address
    #[inline(always)]
    pub fn prtaddr(&self) -> PRTADDR_R {
        PRTADDR_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 7:13 - Hub address
    #[inline(always)]
    pub fn hubaddr(&self) -> HUBADDR_R {
        HUBADDR_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    ///Bits 14:15 - Transaction position
    #[inline(always)]
    pub fn xactpos(&self) -> XACTPOS_R {
        XACTPOS_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - Do complete split
    #[inline(always)]
    pub fn complsplt(&self) -> COMPLSPLT_R {
        COMPLSPLT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 31 - Split enable
    #[inline(always)]
    pub fn spliten(&self) -> SPLITEN_R {
        SPLITEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCSPLT0")
            .field("prtaddr", &self.prtaddr())
            .field("hubaddr", &self.hubaddr())
            .field("xactpos", &self.xactpos())
            .field("complsplt", &self.complsplt())
            .field("spliten", &self.spliten())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Port address
    #[inline(always)]
    pub fn prtaddr(&mut self) -> PRTADDR_W<'_, HCSPLT0rs> {
        PRTADDR_W::new(self, 0)
    }
    ///Bits 7:13 - Hub address
    #[inline(always)]
    pub fn hubaddr(&mut self) -> HUBADDR_W<'_, HCSPLT0rs> {
        HUBADDR_W::new(self, 7)
    }
    ///Bits 14:15 - Transaction position
    #[inline(always)]
    pub fn xactpos(&mut self) -> XACTPOS_W<'_, HCSPLT0rs> {
        XACTPOS_W::new(self, 14)
    }
    ///Bit 16 - Do complete split
    #[inline(always)]
    pub fn complsplt(&mut self) -> COMPLSPLT_W<'_, HCSPLT0rs> {
        COMPLSPLT_W::new(self, 16)
    }
    ///Bit 31 - Split enable
    #[inline(always)]
    pub fn spliten(&mut self) -> SPLITEN_W<'_, HCSPLT0rs> {
        SPLITEN_W::new(self, 31)
    }
}
/**OTG host channel 0 split control register

You can [`read`](crate::Reg::read) this register and get [`hcsplt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#OTG1:HCSPLT0)*/
pub struct HCSPLT0rs;
impl crate::RegisterSpec for HCSPLT0rs {
    type Ux = u32;
}
///`read()` method returns [`hcsplt0::R`](R) reader structure
impl crate::Readable for HCSPLT0rs {}
///`write(|w| ..)` method takes [`hcsplt0::W`](W) writer structure
impl crate::Writable for HCSPLT0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCSPLT0 to value 0
impl crate::Resettable for HCSPLT0rs {}
