///Register `DMARPDR` reader
pub type R = crate::R<DMARPDRrs>;
///Register `DMARPDR` writer
pub type W = crate::W<DMARPDRrs>;
/**Receive poll demand

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum RPD {
    ///0: Poll the receive descriptor list
    Poll = 0,
}
impl From<RPD> for u32 {
    #[inline(always)]
    fn from(variant: RPD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RPD {
    type Ux = u32;
}
impl crate::IsEnum for RPD {}
///Field `RPD` reader - Receive poll demand
pub type RPD_R = crate::FieldReader<RPD>;
impl RPD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RPD> {
        match self.bits {
            0 => Some(RPD::Poll),
            _ => None,
        }
    }
    ///Poll the receive descriptor list
    #[inline(always)]
    pub fn is_poll(&self) -> bool {
        *self == RPD::Poll
    }
}
///Field `RPD` writer - Receive poll demand
pub type RPD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, RPD>;
impl<'a, REG> RPD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    ///Poll the receive descriptor list
    #[inline(always)]
    pub fn poll(self) -> &'a mut crate::W<REG> {
        self.variant(RPD::Poll)
    }
}
impl R {
    ///Bits 0:31 - Receive poll demand
    #[inline(always)]
    pub fn rpd(&self) -> RPD_R {
        RPD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMARPDR").field("rpd", &self.rpd()).finish()
    }
}
impl W {
    ///Bits 0:31 - Receive poll demand
    #[inline(always)]
    pub fn rpd(&mut self) -> RPD_W<'_, DMARPDRrs> {
        RPD_W::new(self, 0)
    }
}
/**EHERNET DMA receive poll demand register

You can [`read`](crate::Reg::read) this register and get [`dmarpdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmarpdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F765.html#Ethernet_DMA:DMARPDR)*/
pub struct DMARPDRrs;
impl crate::RegisterSpec for DMARPDRrs {
    type Ux = u32;
}
///`read()` method returns [`dmarpdr::R`](R) reader structure
impl crate::Readable for DMARPDRrs {}
///`write(|w| ..)` method takes [`dmarpdr::W`](W) writer structure
impl crate::Writable for DMARPDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMARPDR to value 0
impl crate::Resettable for DMARPDRrs {}
