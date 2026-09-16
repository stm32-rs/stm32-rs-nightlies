///Register `DSTS` reader
pub type R = crate::R<DSTSrs>;
///Field `SUSPSTS` reader - Suspend status In device mode, this bit is set as long as a suspend condition is detected on the USB. The core enters the suspended state when there is no activity on the USB data lines for a period of 3 ms. The core comes out of the suspend: When there is an activity on the USB data lines When the application writes to the remote wakeup signaling bit in the OTG_DCTL register (RWUSIG bit in OTG_DCTL).
pub type SUSPSTS_R = crate::BitReader;
///Field `ENUMSPD` reader - Enumerated speed Indicates the speed at which the OTG_HS controller has come up after speed detection through a chirp sequence. Others: reserved
pub type ENUMSPD_R = crate::FieldReader;
///Field `EERR` reader - Erratic error The core sets this bit to report any erratic errors. Due to erratic errors, the OTG_HS controller goes into suspended state and an interrupt is generated to the application with Early suspend bit of the OTG_GINTSTS register (ESUSP bit in OTG_GINTSTS). If the early suspend is asserted due to an erratic error, the application can only perform a soft disconnect recover.
pub type EERR_R = crate::BitReader;
///Field `FNSOF` reader - Frame number of the received SOF
pub type FNSOF_R = crate::FieldReader<u16>;
///Field `DEVLNSTS` reader - Device line status Indicates the current logic level USB data lines. Bit \[23\]: Logic level of D+ Bit \[22\]: Logic level of D-
pub type DEVLNSTS_R = crate::FieldReader;
impl R {
    ///Bit 0 - Suspend status In device mode, this bit is set as long as a suspend condition is detected on the USB. The core enters the suspended state when there is no activity on the USB data lines for a period of 3 ms. The core comes out of the suspend: When there is an activity on the USB data lines When the application writes to the remote wakeup signaling bit in the OTG_DCTL register (RWUSIG bit in OTG_DCTL).
    #[inline(always)]
    pub fn suspsts(&self) -> SUSPSTS_R {
        SUSPSTS_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Enumerated speed Indicates the speed at which the OTG_HS controller has come up after speed detection through a chirp sequence. Others: reserved
    #[inline(always)]
    pub fn enumspd(&self) -> ENUMSPD_R {
        ENUMSPD_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - Erratic error The core sets this bit to report any erratic errors. Due to erratic errors, the OTG_HS controller goes into suspended state and an interrupt is generated to the application with Early suspend bit of the OTG_GINTSTS register (ESUSP bit in OTG_GINTSTS). If the early suspend is asserted due to an erratic error, the application can only perform a soft disconnect recover.
    #[inline(always)]
    pub fn eerr(&self) -> EERR_R {
        EERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 8:21 - Frame number of the received SOF
    #[inline(always)]
    pub fn fnsof(&self) -> FNSOF_R {
        FNSOF_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
    ///Bits 22:23 - Device line status Indicates the current logic level USB data lines. Bit \[23\]: Logic level of D+ Bit \[22\]: Logic level of D-
    #[inline(always)]
    pub fn devlnsts(&self) -> DEVLNSTS_R {
        DEVLNSTS_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSTS")
            .field("suspsts", &self.suspsts())
            .field("enumspd", &self.enumspd())
            .field("eerr", &self.eerr())
            .field("fnsof", &self.fnsof())
            .field("devlnsts", &self.devlnsts())
            .finish()
    }
}
/**OTG device status register

You can [`read`](crate::Reg::read) this register and get [`dsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#OTG_HS:DSTS)*/
pub struct DSTSrs;
impl crate::RegisterSpec for DSTSrs {
    type Ux = u32;
}
///`read()` method returns [`dsts::R`](R) reader structure
impl crate::Readable for DSTSrs {}
///`reset()` method sets DSTS to value 0x10
impl crate::Resettable for DSTSrs {
    const RESET_VALUE: u32 = 0x10;
}
