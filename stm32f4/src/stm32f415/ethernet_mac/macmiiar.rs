///Register `MACMIIAR` reader
pub type R = crate::R<MACMIIARrs>;
///Register `MACMIIAR` writer
pub type W = crate::W<MACMIIARrs>;
///Field `MB` reader - MB
pub type MB_R = crate::BitReader;
///Field `MB` writer - MB
pub type MB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MW` reader - MW
pub type MW_R = crate::BitReader;
///Field `MW` writer - MW
pub type MW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CR` reader - CR
pub type CR_R = crate::FieldReader;
///Field `CR` writer - CR
pub type CR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MR` reader - MR
pub type MR_R = crate::FieldReader;
///Field `MR` writer - MR
pub type MR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `PA` reader - PA
pub type PA_R = crate::FieldReader;
///Field `PA` writer - PA
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 0 - MB
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MW
    #[inline(always)]
    pub fn mw(&self) -> MW_R {
        MW_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:4 - CR
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 6:10 - MR
    #[inline(always)]
    pub fn mr(&self) -> MR_R {
        MR_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 11:15 - PA
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACMIIAR")
            .field("mb", &self.mb())
            .field("mw", &self.mw())
            .field("cr", &self.cr())
            .field("mr", &self.mr())
            .field("pa", &self.pa())
            .finish()
    }
}
impl W {
    ///Bit 0 - MB
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W<'_, MACMIIARrs> {
        MB_W::new(self, 0)
    }
    ///Bit 1 - MW
    #[inline(always)]
    pub fn mw(&mut self) -> MW_W<'_, MACMIIARrs> {
        MW_W::new(self, 1)
    }
    ///Bits 2:4 - CR
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W<'_, MACMIIARrs> {
        CR_W::new(self, 2)
    }
    ///Bits 6:10 - MR
    #[inline(always)]
    pub fn mr(&mut self) -> MR_W<'_, MACMIIARrs> {
        MR_W::new(self, 6)
    }
    ///Bits 11:15 - PA
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W<'_, MACMIIARrs> {
        PA_W::new(self, 11)
    }
}
/**Ethernet MAC MII address register

You can [`read`](crate::Reg::read) this register and get [`macmiiar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmiiar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#Ethernet_MAC:MACMIIAR)*/
pub struct MACMIIARrs;
impl crate::RegisterSpec for MACMIIARrs {
    type Ux = u32;
}
///`read()` method returns [`macmiiar::R`](R) reader structure
impl crate::Readable for MACMIIARrs {}
///`write(|w| ..)` method takes [`macmiiar::W`](W) writer structure
impl crate::Writable for MACMIIARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACMIIAR to value 0
impl crate::Resettable for MACMIIARrs {}
