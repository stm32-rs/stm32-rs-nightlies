///Register `DMATPDR` reader
pub type R = crate::R<DMATPDRrs>;
///Register `DMATPDR` writer
pub type W = crate::W<DMATPDRrs>;
/**Transmit poll demand

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum TPD {
    ///0: Poll the transmit descriptor list
    Poll = 0,
}
impl From<TPD> for u32 {
    #[inline(always)]
    fn from(variant: TPD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TPD {
    type Ux = u32;
}
impl crate::IsEnum for TPD {}
///Field `TPD` reader - Transmit poll demand
pub type TPD_R = crate::FieldReader<TPD>;
impl TPD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TPD> {
        match self.bits {
            0 => Some(TPD::Poll),
            _ => None,
        }
    }
    ///Poll the transmit descriptor list
    #[inline(always)]
    pub fn is_poll(&self) -> bool {
        *self == TPD::Poll
    }
}
///Field `TPD` writer - Transmit poll demand
pub type TPD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, TPD>;
impl<'a, REG> TPD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    ///Poll the transmit descriptor list
    #[inline(always)]
    pub fn poll(self) -> &'a mut crate::W<REG> {
        self.variant(TPD::Poll)
    }
}
impl R {
    ///Bits 0:31 - Transmit poll demand
    #[inline(always)]
    pub fn tpd(&self) -> TPD_R {
        TPD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMATPDR").field("tpd", &self.tpd()).finish()
    }
}
impl W {
    ///Bits 0:31 - Transmit poll demand
    #[inline(always)]
    pub fn tpd(&mut self) -> TPD_W<'_, DMATPDRrs> {
        TPD_W::new(self, 0)
    }
}
/**Ethernet DMA transmit poll demand register

You can [`read`](crate::Reg::read) this register and get [`dmatpdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatpdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#Ethernet_DMA:DMATPDR)*/
pub struct DMATPDRrs;
impl crate::RegisterSpec for DMATPDRrs {
    type Ux = u32;
}
///`read()` method returns [`dmatpdr::R`](R) reader structure
impl crate::Readable for DMATPDRrs {}
///`write(|w| ..)` method takes [`dmatpdr::W`](W) writer structure
impl crate::Writable for DMATPDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMATPDR to value 0
impl crate::Resettable for DMATPDRrs {}
